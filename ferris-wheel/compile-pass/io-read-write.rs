// FIXME: Make me compile! Diff budget: 2 lines.
use std::io;
use std::io::Result;

struct ReadWrapper<T: io::Read> {
    inner: T
}

impl<T: io::Read> io::Read for ReadWrapper<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }
}

fn main() { }
