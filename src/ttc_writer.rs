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
use std::collections::btree_map::Entry;
use std::convert::TryFrom;
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
use crate::types::TableRecord;

pub struct TTCWriter<'a, W: Write + Seek> {
    w: &'a mut W,
    data_to_offset_cache: BTreeMap<Rc<[u8]>, u32>,
    main_checksum: Checksum,
    checksum_adjustment_pos: Option<u64>,
}

impl<'a, W: Write + Seek> TTCWriter<'a, W> {
    pub fn new(w: &'a mut W) -> Self {
        Self {
            w,
            data_to_offset_cache: BTreeMap::new(),
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

        let mut table_records = ttc
            .table_directories
            .iter()
            .map(|sfnt| self.patch_head_table(&sfnt.table_records))
            .collect::<Vec<_>>();

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

        let storage_order = ttc
            .table_directories
            .iter()
            .zip(table_records.iter())
            .map(|(sfnt, table_records)| {
                self.get_table_storage_order(sfnt.sfnt_version, table_records)
            })
            .collect::<Vec<_>>();
        for (sfnt_index, sfnt) in ttc.table_directories.iter().enumerate() {
            self.write_fourcc(sfnt.sfnt_version)?;
            self.write_u16be(table_records[sfnt_index].len().try_into().or_else(|_| {
                bail!(
                    "sfnt {} header: number of tables ({}) exceeds 65535",
                    sfnt_index,
                    table_records[sfnt_index].len()
                );
            })?)?;
            self.write_u16be(sfnt.search_range())?;
            self.write_u16be(sfnt.entry_selector())?;
            self.write_u16be(sfnt.range_shift())?;

            for &table_tag in storage_order[sfnt_index].iter() {
                let table_record = table_records[sfnt_index].get_mut(&table_tag).unwrap();
                table_record.checksum = Checksum::from(table_record.raw_data.as_ref()).get();

                let next_padding = self.get_padding(header_offset);
                match self
                    .data_to_offset_cache
                    .entry(table_record.raw_data.clone())
                {
                    Entry::Occupied(entry) => {
                        table_record.offset = *entry.get();
                    }
                    Entry::Vacant(entry) => {
                        header_offset += next_padding;
                        table_record.offset =
                            *entry.insert(header_offset.try_into().or_else(|_| {
                                bail!(
                                    "sfnt {} table {}: offset (0x{:x}) exceeds 4 GiB",
                                    sfnt_index,
                                    table_tag,
                                    header_offset
                                );
                            })?);
                        header_offset += table_record.raw_data.len();
                    }
                }
            }

            for (&table_tag, table_record) in table_records[sfnt_index].iter() {
                self.write_fourcc(table_tag)?;
                self.write_u32be(table_record.checksum)?;
                self.write_u32be(table_record.offset)?;
                self.write_u32be(table_record.raw_data.len().try_into().or_else(|_| {
                    bail!(
                        "sfnt {} table {}: length ({}) exceeds 4 GiB",
                        sfnt_index,
                        table_tag,
                        table_record.raw_data.len()
                    );
                })?)?;
            }
        }

        for (sfnt_index, storage_order) in storage_order.iter().enumerate() {
            for &table_tag in storage_order.iter() {
                let table_record = table_records[sfnt_index].get(&table_tag).unwrap();
                if usize::try_from(table_record.offset).unwrap() >= data_pos {
                    data_pos += self.write_padding(data_pos)?;
                    assert_eq!(table_record.offset.try_into(), Ok(data_pos));
                    self.write_ttc_table_data(&table_record.raw_data)?;
                    data_pos += table_record.raw_data.len();
                }
            }
        }

        if let Some(dsig_header_pos) = dsig_header_pos {
            data_pos += self.write_padding(data_pos)?;
            self.write_ttc_table_data(&ttc.dsig_data)?;
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
        let mut table_records = self.patch_head_table(&sfnt.table_records);

        self.write_fourcc(sfnt.sfnt_version)?;
        self.write_u16be(table_records.len().try_into().or_else(|_| {
            bail!(
                "sfnt 0 header: number of tables ({}) exceeds 65535",
                table_records.len()
            );
        })?)?;
        self.write_u16be(sfnt.search_range())?;
        self.write_u16be(sfnt.entry_selector())?;
        self.write_u16be(sfnt.range_shift())?;

        let mut header_offset = 12 + table_records.len() * 16;
        let mut data_pos = header_offset;

        let storage_order = self.get_table_storage_order(sfnt.sfnt_version, &table_records);
        for &table_tag in storage_order.iter() {
            let table_record = table_records.get_mut(&table_tag).unwrap();
            table_record.checksum = Checksum::from(table_record.raw_data.as_ref()).get();
            header_offset += self.get_padding(header_offset);
            table_record.offset = header_offset.try_into().or_else(|_| {
                bail!(
                    "sfnt 0 table {}: offset (0x{:x}) exceeds 4 GiB",
                    table_tag,
                    header_offset
                );
            })?;
            header_offset += table_record.raw_data.len();
        }

        for (&table_tag, table_record) in table_records.iter() {
            self.write_fourcc(table_tag)?;
            self.write_u32be(table_record.checksum)?;
            self.write_u32be(table_record.offset)?;
            self.write_u32be(table_record.raw_data.len().try_into().or_else(|_| {
                bail!(
                    "sfnt 0 table {}: length ({}) exceeds 4 GiB",
                    table_tag,
                    table_record.raw_data.len()
                );
            })?)?;
        }

        for &table_tag in storage_order.iter() {
            let table_record = table_records.get(&table_tag).unwrap();
            data_pos += self.write_padding(data_pos)?;
            assert_eq!(table_record.offset.try_into(), Ok(data_pos));
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

    fn get_table_storage_order(
        &self,
        sfnt_version: FourCC,
        table_records: &BTreeMap<FourCC, TableRecord>,
    ) -> Vec<FourCC> {
        const PRIORITY_LIST_OTF: [FourCC; 20] = [
            FourCC(*b"head"),
            FourCC(*b"hhea"),
            FourCC(*b"maxp"),
            FourCC(*b"OS/2"),
            FourCC(*b"hmtx"),
            FourCC(*b"LTSH"),
            FourCC(*b"VDMX"),
            FourCC(*b"hdmx"),
            FourCC(*b"cmap"),
            FourCC(*b"fpgm"),
            FourCC(*b"prep"),
            FourCC(*b"cvt "),
            FourCC(*b"loca"),
            FourCC(*b"glyf"),
            FourCC(*b"kern"),
            FourCC(*b"name"),
            FourCC(*b"post"),
            FourCC(*b"gasp"),
            FourCC(*b"PCLT"),
            FourCC(*b"DSIG"),
        ];
        const PRIORITY_LIST_CFF: [FourCC; 8] = [
            FourCC(*b"head"),
            FourCC(*b"hhea"),
            FourCC(*b"maxp"),
            FourCC(*b"OS/2"),
            FourCC(*b"name"),
            FourCC(*b"cmap"),
            FourCC(*b"post"),
            FourCC(*b"CFF "),
        ];
        let priority_list = if sfnt_version == b"OTTO".into() {
            PRIORITY_LIST_CFF.as_ref()
        } else {
            PRIORITY_LIST_OTF.as_ref()
        };
        priority_list
            .iter()
            .filter(|&table_tag| table_records.contains_key(table_tag))
            .chain(
                table_records
                    .keys()
                    .filter(|&table_tag| !priority_list.contains(table_tag)),
            )
            .copied()
            .collect()
    }

    fn patch_head_table(
        &self,
        table_records: &BTreeMap<FourCC, TableRecord>,
    ) -> BTreeMap<FourCC, TableRecord> {
        table_records
            .iter()
            .map(|(&table_tag, table_record)| {
                (
                    table_tag,
                    if table_tag != b"head".into()
                        || table_record.raw_data.len() <= 12
                        || table_record.raw_data[8..12] == [0; 4]
                    {
                        table_record.clone()
                    } else {
                        let mut raw_data_vec = table_record.raw_data.to_vec();
                        raw_data_vec[8..12].fill(0);
                        TableRecord {
                            checksum: 0,
                            offset: 0,
                            raw_data: Rc::from(raw_data_vec),
                        }
                    },
                )
            })
            .collect()
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

    fn write_ttc_table_data(&mut self, raw_data: &[u8]) -> io::Result<()> {
        self.w.write_all(raw_data)?;
        self.main_checksum.write_all(raw_data).unwrap();
        Ok(())
    }

    fn write_sfnt_table_data(&mut self, table_tag: FourCC, raw_data: &[u8]) -> io::Result<()> {
        if table_tag != b"head".into() || raw_data.len() <= 8 {
            self.w.write_all(raw_data)?;
            self.main_checksum.write_all(raw_data).unwrap();
        } else {
            self.w.write_all(&raw_data[..8])?;
            self.main_checksum.write_all(&raw_data[..8]).unwrap();
            if raw_data.len() >= 12 {
                assert!(self.checksum_adjustment_pos.is_none());
                self.checksum_adjustment_pos = Some(self.w.stream_position()?);
            }
            self.w.write_all(&raw_data[8..])?;
            self.main_checksum.write_all(&raw_data[8..]).unwrap();
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
