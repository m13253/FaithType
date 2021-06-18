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

use std::rc::Rc;

use super::types::FourCC;
use super::types::TTCHeader;
use super::types::TableRecord;

pub fn remove_dsig(ttc: &mut TTCHeader) {
    if ttc.dsig_tag == b"DSIG".into() {
        ttc.dsig_tag = FourCC::zeroed();
        ttc.dsig_data = Rc::from([]);
    }
    for sfnt in ttc.table_directories.iter_mut() {
        sfnt.table_records.remove(&b"DSIG".into());
    }
}

pub fn remove_bitmap(ttc: &mut TTCHeader) {
    for sfnt in ttc.table_directories.iter_mut() {
        sfnt.table_records.remove(&b"bdat".into());
        sfnt.table_records.remove(&b"bloc".into());
        sfnt.table_records.remove(&b"EBDT".into());
        sfnt.table_records.remove(&b"EBLC".into());
        sfnt.table_records.remove(&b"EBSC".into());
    }
}

const PATCHED_PREP: [u8; 15] = [
    0xb1, // PUSHB[1]
    0x04, // value = 4
    0x03, // s = 3
    0x8e, // INSTRCTRL[], turn Microsoft ClearType on
    //
    0xb8, // PUSHW[0]
    0x01, 0xff, // n = 0x01ff, always do dropout control
    0x85, // SCANCTRL[]
    //
    0xb0, // PUSHB[0]
    0x04, // n = 4, smart dropout control scan conversion including stubs
    0x8d, // SCANTYPE[]
    //
    0xb1, // PUSHB[1]
    0x01, // value = 1
    0x01, // s = 1
    0x8e, // INSTRCTRL[], turn grid-fitting off
];

pub fn remove_hinting(ttc: &mut TTCHeader) {
    eprintln!("[ WARN ] You request to remove hinting instructions.  But FaithType does not");
    eprintln!("         know whether there are remaining hinting instructions inside the");
    eprintln!("         “glyf” table due to its high complexity.  Instead, they are simply");
    eprintln!("         disabled and will not cause trouble. If you want to remove cleanly to");
    eprintln!("         reduce the file size, use “ttfautohint --dehint” before this program.");
    for sfnt in ttc.table_directories.iter_mut() {
        sfnt.table_records.remove(&b"cvar".into());
        sfnt.table_records.remove(&b"cvt ".into());
        sfnt.table_records.remove(&b"fpgm".into());
        sfnt.table_records.remove(&b"hdmx".into());
        // The hinting instructions in "glyf" table is not modified due to the high complexity.
        // Instead, the "PUSHB[1] 1 1 INSTRCTRL[]" instruction makes "glyf" instructions useless.
        // Use "ttfautohint --dehint" before this if you are interested in removing them.
        sfnt.table_records.insert(
            b"prep".into(),
            TableRecord {
                checksum: 0,
                offset: 0,
                raw_data: Rc::from(PATCHED_PREP),
            },
        );
        sfnt.table_records.remove(&b"LTSH".into());
        sfnt.table_records.remove(&b"VDMX".into());
    }
}

const PATCHED_GASP: [u8; 8] = [
    0x00, 0x01, // version
    0x00, 0x01, // numRanges
    0xff, 0xff, // gaspRanges[0].rangeMaxPPEM = 65535
    0x00, 0x0a, // gaspRanges[0].rangeGaspBehavior = GASP_DO_GRAY | GASP_SYMMETRIC_SMOOTHING
];

pub fn regenerate_gasp(ttc: &mut TTCHeader) {
    for sfnt in ttc.table_directories.iter_mut() {
        sfnt.table_records.insert(
            b"gasp".into(),
            TableRecord {
                checksum: 0,
                offset: 0,
                raw_data: Rc::from(PATCHED_GASP),
            },
        );
    }
}

pub fn patch_head(ttc: &mut TTCHeader) {
    for sfnt in ttc.table_directories.iter_mut() {
        if sfnt.sfnt_version == b"true".into() {
            sfnt.sfnt_version = [0, 1, 0, 0].into();
        }
        if let Some(head) = sfnt.table_records.get_mut(&b"head".into()) {
            if head.raw_data.len() >= 16 {
                let mut raw_data_copy = head.raw_data.to_vec();
                // flags[bit 11]: font data is lossless converted
                // flags[bit 13]: font optimized for Microsoft ClearType
                raw_data_copy[16] |= 0x28;
                head.raw_data = Rc::from(raw_data_copy);
            }
        }
    }
}
