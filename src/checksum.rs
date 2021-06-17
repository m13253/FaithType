use std::io;
use std::io::Write;

#[derive(Clone, Default)]
pub struct Checksum {
    sum: [u32; 4],
    index: usize,
}

impl Checksum {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<'a>(&mut self, buf: impl IntoIterator<Item = &'a u8>) -> &mut Self {
        for &v in buf {
            self.index %= 4;
            self.sum[self.index] += u32::from(v);
            self.index += 1;
        }
        self
    }

    pub fn get(&self) -> u32 {
        (self.sum[0] << 24)
            .wrapping_add(self.sum[1] << 16)
            .wrapping_add(self.sum[2] << 8)
            .wrapping_add(self.sum[3])
    }
}

impl Write for Checksum {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.push(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl From<&[u8]> for Checksum {
    fn from(buf: &[u8]) -> Self {
        let mut checksum = Self::new();
        checksum.push(buf);
        checksum
    }
}
