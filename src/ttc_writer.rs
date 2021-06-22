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

use std::collections::btree_map::BTreeMap;
use std::convert::TryInto;
use std::io;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::rc::Rc;

use anyhow::bail;
use anyhow::Result;

use super::checksum::Checksum;
use super::types::FourCC;
use super::types::SfntHeader;
use super::types::TTCHeader;

pub struct TTCWriter<'a, W: Write + Seek> {
    w: &'a mut W,
    raw_data_cache: BTreeMap<Rc<[u8]>, usize>,
    main_checksum: Checksum,
    checksum_adjustment_pos: Option<u64>,
}

impl<'a, W: Write + Seek> TTCWriter<'a, W> {
    pub fn new(w: &'a mut W) -> Self {
        Self {
            w,
            raw_data_cache: BTreeMap::new(),
            main_checksum: Checksum::new(),
            checksum_adjustment_pos: None,
        }
    }

    pub fn write_ttc(mut self, ttc: &TTCHeader) -> Result<()> {
        if ttc.table_directories.len() == 1 {
            return self.write_sfnt(&ttc.table_directories[0]);
        }
        if ttc.major_version > 2 {
            bail!(
                "unsupported TTC version: {}.{}",
                ttc.major_version,
                ttc.minor_version
            );
        }

        self.write_fourcc(ttc.ttc_tag)?;
        self.write_u16be(ttc.major_version)?;
        self.write_u16be(ttc.minor_version)?;
        self.write_u32be(ttc.table_directories.len().try_into().or_else(|_| {
            bail!(
                "TTC header: number of sfnt entries ({}) exceeds 2147483647",
                ttc.table_directories.len()
            );
        })?)?;

        let mut header_offset = 12 + 4 * ttc.table_directories.len();
        if ttc.major_version >= 2 {
            header_offset += 12;
        }
        for (index, sfnt) in ttc.table_directories.iter().enumerate() {
            self.write_u32be(header_offset.try_into().or_else(|_| {
                bail!(
                    "sfnt {} header: offset (0x{:x}) exceeds 4 GiB",
                    index,
                    header_offset
                );
            })?)?;
            header_offset += 12 + sfnt.table_records.len() * 16;
        }
        let mut data_pos = header_offset;

        let mut dsig_header_pos = None;
        if ttc.major_version >= 2 {
            self.write_fourcc(ttc.dsig_tag)?;
            self.write_u32be(ttc.dsig_data.len().try_into().or_else(|_| {
                bail!(
                    "TTC table {}: length ({}) exceeds 4 GiB",
                    ttc.dsig_tag,
                    ttc.dsig_data.len()
                );
            })?)?;
            if ttc.dsig_tag != FourCC::zeroed() || !ttc.dsig_data.is_empty() {
                dsig_header_pos = Some(self.w.stream_position()?);
            }
            self.w.write_all(&[0; 4])?;
        }

        for (sfnt_index, sfnt) in ttc.table_directories.iter().enumerate() {
            self.write_fourcc(sfnt.sfnt_version)?;
            self.write_u16be(sfnt.table_records.len().try_into().or_else(|_| {
                bail!(
                    "sfnt {} header: number of tables ({}) exceeds 65535",
                    sfnt_index,
                    sfnt.table_records.len()
                );
            })?)?;
            self.write_u16be(sfnt.search_range())?;
            self.write_u16be(sfnt.entry_selector())?;
            self.write_u16be(sfnt.range_shift())?;

            for (&table_tag, table_record) in sfnt.table_records.iter() {
                self.write_fourcc(table_tag)?;
                self.write_u32be(self.checksum_table(table_tag, &table_record.raw_data))?;

                let raw_data = self.patch_ttc_table(table_tag, &table_record.raw_data);
                if let Some(&cache_offset) = self.raw_data_cache.get(&raw_data) {
                    self.write_u32be(cache_offset.try_into().or_else(|_| {
                        bail!(
                            "sfnt 0 table {}: offset (0x{:x}) exceeds 4 GiB",
                            table_tag,
                            cache_offset
                        );
                    })?)?;
                    self.write_u32be(table_record.raw_data.len().try_into().or_else(|_| {
                        bail!(
                            "sfnt 0 table {}: length ({}) exceeds 4 GiB",
                            table_tag,
                            table_record.raw_data.len()
                        );
                    })?)?;
                } else {
                    header_offset += self.get_padding(header_offset);
                    self.write_u32be(header_offset.try_into().or_else(|_| {
                        bail!(
                            "sfnt 0 table {}: offset (0x{:x}) exceeds 4 GiB",
                            table_tag,
                            header_offset
                        );
                    })?)?;
                    self.raw_data_cache.insert(raw_data, header_offset);
                    self.write_u32be(table_record.raw_data.len().try_into().or_else(|_| {
                        bail!(
                            "sfnt 0 table {}: length ({}) exceeds 4 GiB",
                            table_tag,
                            table_record.raw_data.len()
                        );
                    })?)?;
                    header_offset += table_record.raw_data.len();
                }
            }
        }

        for sfnt in ttc.table_directories.iter() {
            for (&table_tag, table_record) in sfnt.table_records.iter() {
                let raw_data = self.patch_ttc_table(table_tag, &table_record.raw_data);
                let cache_offset = *self.raw_data_cache.get(&raw_data).unwrap();
                if cache_offset >= data_pos {
                    data_pos += self.write_padding(data_pos)?;
                    assert_eq!(cache_offset, data_pos);
                    self.write_ttc_table_data(table_tag, &table_record.raw_data)?;
                    data_pos += table_record.raw_data.len();
                }
            }
        }

        if let Some(dsig_header_pos) = dsig_header_pos {
            data_pos += self.write_padding(data_pos)?;
            self.write_ttc_table_data(ttc.dsig_tag, &ttc.dsig_data)?;
            self.w.seek(SeekFrom::Start(dsig_header_pos))?;
            self.write_u32be(data_pos.try_into().or_else(|_| {
                bail!(
                    "TTC table {}: offset (0x{:x}) exceeds 4 GiB",
                    ttc.dsig_tag,
                    data_pos
                );
            })?)?;
        }

        Ok(())
    }

    pub fn write_sfnt(mut self, sfnt: &SfntHeader) -> Result<()> {
        self.write_fourcc(sfnt.sfnt_version)?;
        self.write_u16be(sfnt.table_records.len().try_into().or_else(|_| {
            bail!(
                "sfnt 0 header: number of tables ({}) exceeds 65535",
                sfnt.table_records.len()
            );
        })?)?;
        self.write_u16be(sfnt.search_range())?;
        self.write_u16be(sfnt.entry_selector())?;
        self.write_u16be(sfnt.range_shift())?;

        let mut header_offset = 12 + sfnt.table_records.len() * 16;
        let mut data_pos = header_offset;
        for (&table_tag, table_record) in sfnt.table_records.iter() {
            self.write_fourcc(table_tag)?;
            self.write_u32be(self.checksum_table(table_tag, &table_record.raw_data))?;

            header_offset += self.get_padding(header_offset);
            self.write_u32be(header_offset.try_into().or_else(|_| {
                bail!(
                    "sfnt 0 table {}: offset (0x{:x}) exceeds 4 GiB",
                    table_tag,
                    header_offset
                );
            })?)?;
            self.write_u32be(table_record.raw_data.len().try_into().or_else(|_| {
                bail!(
                    "sfnt 0 table {}: length ({}) exceeds 4 GiB",
                    table_tag,
                    table_record.raw_data.len()
                );
            })?)?;
            header_offset += table_record.raw_data.len();
        }

        for (&table_tag, table_record) in sfnt.table_records.iter() {
            data_pos += self.write_padding(data_pos)?;
            self.write_sfnt_table_data(table_tag, &table_record.raw_data)?;
            data_pos += table_record.raw_data.len();
        }

        if let Some(checksum_adjustment_pos) = self.checksum_adjustment_pos {
            self.w.seek(SeekFrom::Start(checksum_adjustment_pos))?;
            let main_checksum = self.main_checksum.get();
            self.write_u32be(0xb1b0afba_u32.wrapping_sub(main_checksum))?;
        }

        Ok(())
    }

    fn checksum_table(&self, table_tag: FourCC, raw_data: &[u8]) -> u32 {
        if table_tag != b"head".into() || raw_data.len() <= 8 {
            Checksum::from(raw_data).get()
        } else {
            let mut checksum = Checksum::from(&raw_data[..8]);
            if raw_data.len() >= 12 {
                checksum.push(&raw_data[12..]);
            }
            checksum.get()
        }
    }

    fn patch_ttc_table(&self, table_tag: FourCC, raw_data: &Rc<[u8]>) -> Rc<[u8]> {
        if table_tag != b"head".into() || raw_data.len() <= 12 || raw_data[8..12] == [0; 4] {
            raw_data.clone()
        } else {
            let mut raw_data_vec = raw_data.to_vec();
            raw_data_vec[8..12].fill(0);
            Rc::from(raw_data_vec)
        }
    }

    fn write_u16be(&mut self, value: u16) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.w.write_all(&buf)?;
        self.main_checksum.write_all(&buf).unwrap();
        Ok(())
    }

    fn write_u32be(&mut self, value: u32) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.w.write_all(&buf)?;
        self.main_checksum.write_all(&buf).unwrap();
        Ok(())
    }

    fn write_fourcc(&mut self, value: FourCC) -> io::Result<()> {
        self.w.write_all(&value.0)?;
        self.main_checksum.write_all(&value.0).unwrap();
        Ok(())
    }

    fn write_ttc_table_data(&mut self, table_tag: FourCC, raw_data: &[u8]) -> io::Result<()> {
        if table_tag != b"head".into() || raw_data.len() <= 8 {
            self.w.write_all(raw_data)?;
            self.main_checksum.write_all(raw_data).unwrap();
        } else {
            self.w.write_all(&raw_data[..8])?;
            self.main_checksum.write_all(&raw_data[..8]).unwrap();
            if raw_data.len() < 12 {
                self.w.write_all(&raw_data[8..])?;
                self.main_checksum.write_all(&raw_data[8..]).unwrap();
            } else {
                self.w.write_all(&[0; 4])?;
                self.w.write_all(&raw_data[12..])?;
                self.main_checksum.write_all(&raw_data[12..]).unwrap();
            }
        }
        Ok(())
    }

    fn write_sfnt_table_data(&mut self, table_tag: FourCC, raw_data: &[u8]) -> io::Result<()> {
        if table_tag != b"head".into() || raw_data.len() <= 8 {
            self.w.write_all(raw_data)?;
            self.main_checksum.write_all(raw_data).unwrap();
        } else {
            self.w.write_all(&raw_data[..8])?;
            self.main_checksum.write_all(&raw_data[..8]).unwrap();
            if raw_data.len() < 12 {
                self.w.write_all(&raw_data[8..])?;
                self.main_checksum.write_all(&raw_data[8..]).unwrap();
            } else {
                assert!(self.checksum_adjustment_pos.is_none());
                self.checksum_adjustment_pos = Some(self.w.stream_position()?);
                self.w.write_all(&[0; 4])?;
                self.w.write_all(&raw_data[12..])?;
                self.main_checksum.write_all(&raw_data[12..]).unwrap();
            }
        }
        Ok(())
    }

    fn write_padding(&mut self, pos: usize) -> io::Result<usize> {
        let padding_size = self.get_padding(pos);
        let padding = &[0; 3][..padding_size];
        self.w.write_all(padding)?;
        self.main_checksum.write_all(padding).unwrap();
        Ok(padding_size)
    }

    fn get_padding(&self, pos: usize) -> usize {
        pos.wrapping_neg() & 3
    }
}
