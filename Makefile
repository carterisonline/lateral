SHELL := /usr/bin/bash
SPEC := ./spec
QEMU_OPTIONS := -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio
PROJECT_NAME := lateral

ifdef VERBOSE
  Q :=
else
  Q := @
endif

BOOTIMAGE := $(shell cargo install bootimage)
LLVM_TOOLS_PREVIEW := $(shell rustup component add llvm-tools-preview)

help:
	$(Q)echo 'make clean                  removes ./target, ./doc, and Cargo.lock.'
	$(Q)echo 'make dev                    compiles in development mode.'
	$(Q)echo 'make release [ARCH]         compiles in release mode.'
	$(Q)echo 'make run                    compiles and runs output in development mode.'
	$(Q)echo 'make run-release [ARCH]     compiles and runs output in release mode.'

clean:
	$(Q)cargo clean
	$(Q)rm -rf "doc/"
	$(Q)rm -f Cargo.lock

dev:
	$(Q)cargo bootimage --target ${SPEC}/${ARCH}-${PROJECT_NAME}.json

release:
	$(Q)cargo bootimage --release --target ${SPEC}/${ARCH}-${PROJECT_NAME}.json

run:
	$(Q)cargo bootimage --target ${SPEC}/${ARCH}-${PROJECT_NAME}.json
	qemu-system-${ARCH} -drive format=raw,file=target/${ARCH}-${PROJECT_NAME}/debug/bootimage-${PROJECT_NAME}.bin ${QEMU_OPTIONS}

run-release:
	$(Q)cargo bootimage --release --target ${SPEC}/${ARCH}-${PROJECT_NAME}.json
	qemu-system-${ARCH} -drive format=raw,file=target/${ARCH}-${PROJECT_NAME}/release/bootimage-${PROJECT_NAME}.bin ${QEMU_OPTIONS}

test:
	cargo test --target ${SPEC}/${ARCH}-${PROJECT_NAME}.json