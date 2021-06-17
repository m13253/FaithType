use anyhow::bail;
use anyhow::Result;
use std::collections::hash_map::Entry;
use std::collections::hash_map::HashMap;
use std::convert::TryInto;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::rc::Rc;

use super::types::FourCC;
use super::types::SfntHeader;
use super::types::TTCHeader;
use super::types::TableRecord;

pub struct TTCReader<'a, R: Read + Seek> {
    r: &'a mut R,
    raw_data_cache: HashMap<(u64, usize), Rc<[u8]>>,
}

impl<'a, R: Read + Seek> TTCReader<'a, R> {
    pub fn new(r: &'a mut R) -> Self {
        Self {
            r,
            raw_data_cache: HashMap::new(),
        }
    }

    pub fn read_ttc(mut self) -> Result<TTCHeader> {
        let old_pos = self.r.stream_position()?;
        let ttc_tag = self.read_fourcc()?;
        if ttc_tag != b"ttcf".into() {
            self.r.seek(SeekFrom::Start(old_pos))?;
            let sfnt = self.read_sfnt()?;
            return Ok(TTCHeader {
                ttc_tag: b"ttcf".into(),
                major_version: 1,
                minor_version: 0,
                table_directories: vec![sfnt],
                dsig_tag: FourCC::zeroed(),
                dsig_data: self.empty_raw_data(),
            });
        }

        let major_version = self.read_u16be()?;
        let minor_version = self.read_u16be()?;
        if major_version > 2 {
            bail!(
                "file position 0x{:08x}: unsupported TTC version: {}.{}",
                self.r.stream_position()? - 4,
                major_version,
                minor_version
            );
        }
        let num_fonts = self.read_u32be()?;
        let table_directories = (0..num_fonts)
            .map(|_| {
                let offset = self.read_u32be()?;
                let old_pos = self.r.stream_position()?;
                self.r.seek(SeekFrom::Start(offset.into()))?;
                let sfnt = self.read_sfnt()?;
                self.r.seek(SeekFrom::Start(old_pos))?;
                Ok(sfnt)
            })
            .collect::<Result<_>>()?;
        let dsig_tag;
        let dsig_length;
        let dsig_offset;
        if major_version < 2 {
            dsig_tag = FourCC::zeroed();
            dsig_length = 0;
            dsig_offset = 0;
        } else {
            dsig_tag = self.read_fourcc()?;
            dsig_length = self.read_u32be()?;
            dsig_offset = self.read_u32be()?;
        }
        let dsig_data = if dsig_tag == b"DSIG".into() {
            self.read_raw_data(dsig_offset.into(), dsig_length.try_into().unwrap())?
        } else {
            self.empty_raw_data()
        };

        Ok(TTCHeader {
            ttc_tag,
            major_version,
            minor_version,
            table_directories,
            dsig_tag,
            dsig_data,
        })
    }

    fn read_sfnt(&mut self) -> Result<SfntHeader> {
        let sfnt_version = self.read_fourcc()?;
        match &sfnt_version.0 {
            // Microsoft OpenType font
            &[0, 1, 0, 0] => (),
            // Adobe CFF font
            b"OTTO" => (),
            // Apple TrueType font
            b"true" => (),
            // sfnt can also contain other formats
            _ => bail!(
                "file position 0x{:08x}: unsupported sfnt version: {}",
                self.r.stream_position()? - 4,
                format_args!("{}", sfnt_version)
            ),
        }

        let num_tables = self.read_u16be()?;
        let _search_range = self.read_u16be()?;
        let _entry_selector = self.read_u16be()?;
        let _range_shift = self.read_u16be()?;
        let table_records = (0..num_tables)
            .map(|_| {
                let table_tag = self.read_fourcc()?;
                let checksum = self.read_u32be()?;
                let offset = self.read_u32be()?;
                let length = self.read_u32be()?;
                let raw_data = self.read_raw_data(offset.into(), length.try_into().unwrap())?;
                Ok((
                    table_tag,
                    TableRecord {
                        checksum,
                        offset,
                        raw_data,
                    },
                ))
            })
            .collect::<Result<_>>()?;

        Ok(SfntHeader {
            sfnt_version,
            table_records,
        })
    }

    fn read_u16be(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.r.read_exact(&mut buf)?;
        Ok((buf[0] as u16) << 8 | (buf[1] as u16))
    }

    fn read_u32be(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.r.read_exact(&mut buf)?;
        Ok((buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32))
    }

    fn read_fourcc(&mut self) -> Result<FourCC> {
        let mut buf = [0; 4];
        self.r.read_exact(&mut buf)?;
        Ok(FourCC(buf))
    }

    fn read_raw_data(&mut self, pos: u64, len: usize) -> Result<Rc<[u8]>> {
        if len == 0 {
            return Ok(self.empty_raw_data());
        }

        match self.raw_data_cache.entry((pos, len)) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let old_pos = self.r.stream_position()?;
                self.r.seek(SeekFrom::Start(pos))?;
                let mut buf = vec![0; len];
                self.r.read_exact(&mut buf)?;
                self.r.seek(SeekFrom::Start(old_pos))?;
                Ok(entry.insert(Rc::from(buf)).clone())
            }
        }
    }

    fn empty_raw_data(&mut self) -> Rc<[u8]> {
        match self.raw_data_cache.entry((0, 0)) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => entry.insert(Rc::new([])).clone(),
        }
    }
}
