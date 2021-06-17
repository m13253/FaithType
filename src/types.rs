use std::ascii;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FourCC(pub [u8; 4]);

#[derive(Clone)]
pub struct TTCHeader {
    pub ttc_tag: FourCC,
    pub major_version: u16,
    pub minor_version: u16,
    pub table_directories: Vec<SfntHeader>,
    pub dsig_tag: FourCC,
    pub dsig_data: Rc<[u8]>,
}

#[derive(Clone)]
pub struct SfntHeader {
    pub sfnt_version: FourCC,
    pub table_records: BTreeMap<FourCC, TableRecord>,
}

#[derive(Clone)]
pub struct TableRecord {
    pub checksum: u32,
    pub raw_data: Rc<[u8]>,
}

impl FourCC {
    pub const fn new(bytes: &'static [u8; 4]) -> Self {
        Self(*bytes)
    }

    pub const fn zeroed() -> Self {
        Self([0; 4])
    }
}

impl SfntHeader {
    pub fn search_range(&self) -> u16 {
        if self.table_records.len() >= 4096 {
            return 32768;
        }
        16 << self.log2_num_tables()
    }

    pub fn entry_selector(&self) -> u16 {
        self.log2_num_tables().try_into().unwrap()
    }

    pub fn range_shift(&self) -> u16 {
        if self.table_records.len() >= 6144 {
            return 65520;
        }
        if self.table_records.len() >= 4096 {
            return ((self.table_records.len() << 4) - 32768)
                .try_into()
                .unwrap();
        }
        ((self.table_records.len() << 4) - (16 << self.log2_num_tables()))
            .try_into()
            .unwrap()
    }
    fn log2_num_tables(&self) -> u32 {
        let mut len = self.table_records.len();
        let mut result = 0;
        while len != 0 {
            len >>= 1;
            result += 1;
        }
        result
    }
}

impl Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\"")?;
        for c in self.0.iter().copied().flat_map(ascii::escape_default) {
            f.write_char(c.into())?;
        }
        f.write_str("\"")?;
        Ok(())
    }
}

impl Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\"")?;
        for c in self.0.iter().copied().flat_map(ascii::escape_default) {
            f.write_char(c.into())?;
        }
        f.write_str("\"")?;
        Ok(())
    }
}

impl Debug for TTCHeader {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("TTC")
            .field("ttcTag", &format_args!("{}", self.ttc_tag))
            .field(
                "version",
                &format_args!("{}.{}", self.major_version, self.minor_version),
            )
            .field("tableDirectory", &self.table_directories)
            .field("dsigTag", &format_args!("{}", self.dsig_tag))
            .field(
                "dsigData",
                &format_args!("({} bytes)", self.dsig_data.len()),
            )
            .finish()
    }
}

impl Debug for SfntHeader {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("sfnt")
            .field("sfntVersion", &format_args!("{}", self.sfnt_version))
            .field("tableRecords", &self.table_records)
            .finish()
    }
}

impl Debug for TableRecord {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("tableRecord")
            .field("checksum", &format_args!("0x{:08x}", self.checksum))
            .field("data", &format_args!("({} bytes)", self.raw_data.len()))
            .finish()
    }
}
