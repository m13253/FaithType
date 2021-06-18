// FaithType
// Copyright (C) 2021  Star Brilliant <coder@poorlab.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
