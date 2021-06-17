use std::rc::Rc;

use crate::types::FourCC;
use crate::types::TTCHeader;
use crate::types::TableRecord;

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
        sfnt.table_records.remove(&b"EBDT".into());
        sfnt.table_records.remove(&b"EBLC".into());
        sfnt.table_records.remove(&b"EBSC".into());
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
