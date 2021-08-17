#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn square(size: usize) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl core::ops::Sub<usize> for Size {
    type Output = Size;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            width: self.width - rhs,
            height: self.height - rhs,
        }
    }
}

impl core::ops::Sub<Size> for Size {
    type Output = Size;

    fn sub(self, rhs: Size) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}