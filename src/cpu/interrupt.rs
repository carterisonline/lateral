use crate::cpu::gdt;
use crate::halt_loop;
use crate::io::logging::kernel_error;
use crate::syscall::dispatcher;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use rust_alloc::format;
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use core::arch::naked_asm;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

const PIC1: u16 = 0x21;
const PIC2: u16 = 0xA1;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    pub static ref IRQ_HANDLERS: Mutex<[fn(); 16]> = Mutex::new([default_irq_handler; 16]);
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[interrupt_index(0) as usize].set_handler_fn(irq0_handler);
        idt[interrupt_index(1) as usize].set_handler_fn(irq1_handler);
        idt[interrupt_index(2) as usize].set_handler_fn(irq2_handler);
        idt[interrupt_index(3) as usize].set_handler_fn(irq3_handler);
        idt[interrupt_index(4) as usize].set_handler_fn(irq4_handler);
        idt[interrupt_index(5) as usize].set_handler_fn(irq5_handler);
        idt[interrupt_index(6) as usize].set_handler_fn(irq6_handler);
        idt[interrupt_index(7) as usize].set_handler_fn(irq7_handler);
        idt[interrupt_index(8) as usize].set_handler_fn(irq8_handler);
        idt[interrupt_index(9) as usize].set_handler_fn(irq9_handler);
        idt[interrupt_index(10) as usize].set_handler_fn(irq10_handler);
        idt[interrupt_index(11) as usize].set_handler_fn(irq11_handler);
        idt[interrupt_index(12) as usize].set_handler_fn(irq12_handler);
        idt[interrupt_index(13) as usize].set_handler_fn(irq13_handler);
        idt[interrupt_index(14) as usize].set_handler_fn(irq14_handler);
        idt[interrupt_index(15) as usize].set_handler_fn(irq15_handler);
        idt[0x80].set_handler_fn(unsafe {
            core::mem::transmute::<
                *mut fn(),
                extern "x86-interrupt" fn(x86_64::structures::idt::InterruptStackFrame),
            >(wrapped_syscall_handler as *mut fn())
        });
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

macro_rules! irq_handler {
    ($handler:ident, $irq:expr) => {
        pub extern "x86-interrupt" fn $handler(_stack_frame: InterruptStackFrame) {
            let handlers = IRQ_HANDLERS.lock();
            handlers[$irq]();
            unsafe {
                crate::cpu::interrupt::PICS
                    .lock()
                    .notify_end_of_interrupt(interrupt_index($irq));
            }
        }
    };
}

macro_rules! wrap {
    ($fn: ident => $w:ident) => {
        #[naked]
        /// # Safety
        /// lmao
        pub unsafe extern "sysv64" fn $w() {
            naked_asm!(
                "
                push rbp
                push rax
                push rbx
                push rcx
                push rdx
                push rsi
                push rdi
                push r8
                push r9
                push r10
                push r11
                push r12
                push r13
                push r14
                push r15
                mov rsi, rsp  // arg2: register list
                mov rdi, rsp
                add rdi, 15*8 // arg1: interupt frame
                call {}
                pop r15
                pop r14
                pop r13
                pop r12
                pop r11
                pop r10
                pop r9
                pop r8
                pop rdi
                pop rsi
                pop rdx
                pop rcx
                pop rbx
                pop rax
                pop rbp
                iretq
                ",
                sym $fn,
            );
        }
    };
}

extern "sysv64" fn syscall_handler(_stack_frame: &mut InterruptStackFrame, regs: &mut Registers) {
    let n = regs.rax;
    let arg1 = regs.rdi;
    let arg2 = regs.rsi;
    let arg3 = regs.rdx;
    regs.rax = dispatcher(n, arg1, arg2, arg3);
    unsafe { PICS.lock().notify_end_of_interrupt(0x80) };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    kernel_error(format!("BREAKPOINT\n{:#?}", stack_frame).as_str());
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    kernel_error("PAGE FAULT");
    kernel_error(format!("Accessed Address: {:?}", Cr2::read()).as_str());
    kernel_error(format!("Error Code: {:?}", error_code).as_str());
    kernel_error(format!("{:#?}", stack_frame).as_str());
    halt_loop();
}

pub fn init_idt() {
    IDT.load();
}

pub fn set_irq_handler(irq: u8, handler: fn()) {
    without_interrupts(|| {
        let mut handlers = IRQ_HANDLERS.lock();
        handlers[irq as usize] = handler;

        clear_irq_mask(irq);
    });
}

pub fn set_irq_mask(irq: u8) {
    let mut port: Port<u8> = Port::new(if irq < 8 { PIC1 } else { PIC2 });
    unsafe {
        let value = port.read() | (1 << (if irq < 8 { irq } else { irq - 8 }));
        port.write(value);
    }
}

pub fn clear_irq_mask(irq: u8) {
    let mut port: Port<u8> = Port::new(if irq < 8 { PIC1 } else { PIC2 });
    unsafe {
        let value = port.read() & !(1 << if irq < 8 { irq } else { irq - 8 });
        port.write(value);
    }
}

fn interrupt_index(irq: u8) -> u8 {
    crate::cpu::interrupt::PIC_1_OFFSET + irq
}

fn default_irq_handler() {}

wrap!(syscall_handler => wrapped_syscall_handler);

irq_handler!(irq0_handler, 0);
irq_handler!(irq1_handler, 1);
irq_handler!(irq2_handler, 2);
irq_handler!(irq3_handler, 3);
irq_handler!(irq4_handler, 4);
irq_handler!(irq5_handler, 5);
irq_handler!(irq6_handler, 6);
irq_handler!(irq7_handler, 7);
irq_handler!(irq8_handler, 8);
irq_handler!(irq9_handler, 9);
irq_handler!(irq10_handler, 10);
irq_handler!(irq11_handler, 11);
irq_handler!(irq12_handler, 12);
irq_handler!(irq13_handler, 13);
irq_handler!(irq14_handler, 14);
irq_handler!(irq15_handler, 15);

#[repr(align(8), C)]
#[derive(Debug, Clone, Default)]
pub struct Registers {
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    r11: usize,
    r10: usize,
    r9: usize,
    r8: usize,
    rdi: usize,
    rsi: usize,
    rdx: usize,
    rcx: usize,
    rbx: usize,
    rax: usize,
    rbp: usize,
}
