use core::cmp;
use core::fmt;
use core::iter::Take;
use core::str::Bytes;

pub struct ByteCompare;

impl ByteCompare {
    pub fn starts_with<T: fmt::Display>(original: T, prefix: &str) -> bool {
        use core::fmt::Write;
        let mut b = StartsWith(prefix.bytes());
        write!(&mut b, "{}", original).is_ok() && b.is_empty()
    }
}

struct StartsWith<'a>(Bytes<'a>);

impl<'a> StartsWith<'a> {
    fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn take(&mut self, n: usize) -> Take<&mut Bytes<'a>> {
        self.0.by_ref().take(n)
    }
}

impl fmt::Write for StartsWith<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let n = cmp::min(self.len(), s.len());
        self.take(n)
            .eq(s.bytes().take(n))
            .then_some(())
            .ok_or(fmt::Error)
    }
}
