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

use std::borrow::Cow;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::bail;

use super::types::FourCC;
use super::types::TTCHeader;
use super::types::TableRecord;

pub fn remove_dsig(ttc: &mut TTCHeader) {
    const PATCHED_DSIG: [u8; 8] = [
        0x00, 0x00, 0x00, 0x01, // version
        0x00, 0x00, // numSignatures
        0x00, 0x00, // flags
    ];

    if ttc.table_directories.len() == 1 {
        ttc.dsig_tag = FourCC::zeroed();
        ttc.dsig_data = Rc::from([]);
        for sfnt in ttc.table_directories.iter_mut() {
            sfnt.table_records.insert(
                b"DSIG".into(),
                TableRecord {
                    checksum: 0,
                    offset: 0,
                    raw_data: Rc::from(PATCHED_DSIG),
                },
            );
        }
    } else {
        if (ttc.major_version as u32) << 16 | (ttc.minor_version as u32) < 0x0200 {
            ttc.major_version = 2;
            ttc.minor_version = 0;
        }
        ttc.dsig_tag = b"DSIG".into();
        ttc.dsig_data = Rc::from(PATCHED_DSIG);
        for sfnt in ttc.table_directories.iter_mut() {
            sfnt.table_records.remove(&b"DSIG".into());
        }
    }
}

pub fn remove_bitmap(ttc: &mut TTCHeader) {
    for sfnt in ttc.table_directories.iter_mut() {
        // Bitmap data (Apple format)
        sfnt.table_records.remove(&b"bdat".into());
        // Bitmap index (Apple format)
        sfnt.table_records.remove(&b"bloc".into());
        // Bitmap data (Microsoft format)
        sfnt.table_records.remove(&b"EBDT".into());
        // Bitmap index (Microsoft format)
        sfnt.table_records.remove(&b"EBLC".into());
        // Bitmap scaling data (Microsoft format)
        sfnt.table_records.remove(&b"EBSC".into());
    }
}

pub fn remove_hinting(ttc: &mut TTCHeader) {
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

    for (sfnt_index, sfnt) in ttc.table_directories.iter_mut().enumerate() {
        // CVT variations
        sfnt.table_records.remove(&b"cvar".into());
        // Control value table
        sfnt.table_records.remove(&b"cvt ".into());
        // Font program (only run once)
        sfnt.table_records.remove(&b"fpgm".into());
        // Horizontal device metrics
        sfnt.table_records.remove(&b"hdmx".into());
        // CVT Program (run whenever transform matrix changes)
        sfnt.table_records.insert(
            b"prep".into(),
            TableRecord {
                checksum: 0,
                offset: 0,
                raw_data: Rc::from(PATCHED_PREP),
            },
        );
        // Maximum profile
        if let Some(maxp) = sfnt.table_records.get_mut(&b"maxp".into()) {
            let mut raw_data_copy = maxp.raw_data.to_vec();

            // Byte 14..16: maxZones, 1 if instructions do not use the twilight zone (Z0)
            raw_data_copy
                .get_mut(14..16)
                .map(|x| x.clone_from_slice(&[0, 1]));
            // Byte 16..18: maxTwilightPoints
            // Byte 18..20: maxStorage
            // Byte 20..22: maxFunctionDefs
            // Byte 22..24: maxInstructionDefs
            raw_data_copy.get_mut(16..24).map(|x| x.fill(0));
            // Byte 24..26: maxStackElements
            raw_data_copy
                .get_mut(24..26)
                .map(|x| x.clone_from_slice(&[0, 2]));
            // Byte 26..28: maxSizeOfInstructions
            raw_data_copy
                .get_mut(26..28)
                .map(|x| x.clone_from_slice(&[0, PATCHED_PREP.len().try_into().unwrap()]));

            maxp.raw_data = Rc::from(raw_data_copy);
        }
        // Linear Threshold data
        sfnt.table_records.remove(&b"LTSH".into());
        // Vertical Device Metrics
        sfnt.table_records.remove(&b"VDMX".into());

        // "head" table:
        // byte 50..52: indexToLocFormat
        // byte 52..54: glyphDataFormat
        let (loca_format, glyf_format) =
            sfnt.table_records
                .get(&b"head".into())
                .map_or((None, None), |head| {
                    (
                        head.raw_data
                            .get(50..52)
                            .and_then(|x| <[_; 2]>::try_from(x).ok()),
                        head.raw_data
                            .get(52..54)
                            .and_then(|x| <[_; 2]>::try_from(x).ok()),
                    )
                });

        // If there is a "loca" table
        let loca = sfnt
            .table_records
            .get(&b"loca".into())
            .map_or(Vec::new(), |loca| match loca_format {
                Some([0, 0]) => {
                    if loca.raw_data.len() % 2 != 0 {
                        eprintln!(
                            "[ WARN ] sfnt {} table “loca”: length {} is not multiples of 2",
                            sfnt_index,
                            loca.raw_data.len()
                        );
                    }
                    loca.raw_data
                        .chunks_exact(2)
                        .map(|x| {
                            usize::try_from(u16::from_be_bytes(x.try_into().unwrap())).unwrap() * 2
                        })
                        .collect::<Vec<_>>()
                }
                Some([0, 1]) => {
                    if loca.raw_data.len() % 4 != 0 {
                        eprintln!(
                            "[ WARN ] sfnt {} table “loca”: length {} is not multiples of 4",
                            sfnt_index,
                            loca.raw_data.len()
                        );
                    }
                    loca.raw_data
                        .chunks_exact(4)
                        .map(|x| {
                            usize::try_from(u32::from_be_bytes(x.try_into().unwrap())).unwrap()
                        })
                        .collect::<Vec<_>>()
                }
                Some(loca_format) => {
                    eprintln!(
                        "[ WARN ] sfnt {} table “head”: unsupported indexToLocFormat: {}",
                        sfnt_index,
                        u16::from_be_bytes(loca_format)
                    );
                    Vec::new()
                }
                _ => {
                    eprintln!(
                        "[ WARN ] sfnt {} table “head”: unspecified indexToLocFormat value",
                        sfnt_index
                    );
                    Vec::new()
                }
            });

        // If there is a "glyf" table
        let raw_glyf = sfnt
            .table_records
            .get(&b"glyf".into())
            .map_or([].as_ref(), |glyf| &glyf.raw_data);

        // Extract each glyph from the "glyf" table
        let glyf = loca
            .windows(2)
            .enumerate()
            .map(|(glyph_index, glyph_offset)| {
                let glyph_offset_from = glyph_offset[0];
                let glyph_offset_to = glyph_offset[1];
                if glyph_offset_from > glyph_offset_to {
                    eprintln!(
                        "[ WARN ] sfnt {} table “loca”: glyph {} has a negative length ({} > {})",
                        sfnt_index, glyph_index, glyph_offset_from, glyph_offset_to
                    );
                    [].as_ref()
                } else {
                    &raw_glyf[glyph_offset_from..glyph_offset_to]
                }
            })
            .collect::<Vec<_>>();

        // Make sure we don't run into unknown formats
        if !glyf.is_empty() {
            match glyf_format {
                Some([0, 0]) => (),
                Some(glyf_format) => {
                    eprintln!(
                        "[ WARN ] sfnt {} table “head”: unsupported glyphDataFormat: {}",
                        sfnt_index,
                        u16::from_be_bytes(glyf_format)
                    );
                    continue;
                }
                _ => {
                    eprintln!(
                        "[ WARN ] sfnt {} table “head”: unspecified glyphDataFormat value",
                        sfnt_index
                    );
                    continue;
                }
            }
        }

        // Generate the new "glyf" table
        let mut glyf_modified = false;
        let new_glyf = glyf
            .into_iter()
            .enumerate()
            .map(|(glyph_index, glyph)| {
                if glyph.is_empty() {
                    return Cow::Borrowed(glyph);
                }
                (|| {
                    let num_of_contours = u16::from_be_bytes(
                        glyph
                            .get(..2)
                            .ok_or_else(|| {
                                anyhow!(
                                    "sfnt {} table “glyf”: glyph {} data truncated (0..2)",
                                    sfnt_index,
                                    glyph_index,
                                )
                            })?
                            .try_into()?,
                    );
                    let new_glyph = if num_of_contours < 0x8000 {
                        // Simple glyph
                        let num_of_contours = usize::try_from(num_of_contours).unwrap();
                        let num_of_points = if num_of_contours == 0 {
                            0
                        } else {
                            1 + usize::try_from(u16::from_be_bytes(
                                glyph
                                    .get(8 + num_of_contours * 2..10 + num_of_contours * 2)
                                    .ok_or_else(|| {
                                        anyhow!(
                                        "sfnt {} table “glyf”: glyph {} data truncated ({}..{})",
                                        sfnt_index,
                                        glyph_index,
                                        8 + num_of_contours * 2,
                                        10 + num_of_contours * 2
                                    )
                                    })?
                                    .try_into()?,
                            ))
                            .unwrap()
                        };
                        let instruction_len = usize::try_from(u16::from_be_bytes(
                            glyph
                                .get(10 + num_of_contours * 2..12 + num_of_contours * 2)
                                .ok_or_else(|| {
                                    anyhow!(
                                        "sfnt {} table “glyf”: glyph {} data truncated ({}..{})",
                                        sfnt_index,
                                        glyph_index,
                                        10 + num_of_contours * 2,
                                        12 + num_of_contours * 2
                                    )
                                })?
                                .try_into()?,
                        ))
                        .unwrap();

                        let mut flags_pos = 12 + num_of_contours * 2 + instruction_len;
                        let mut glyph_len = 0;
                        let mut i = 0;
                        while i < num_of_points {
                            let flags = glyph.get(flags_pos).ok_or_else(|| {
                                anyhow!(
                                    "sfnt {} table “glyf”: glyph {} data truncated ({})",
                                    sfnt_index,
                                    glyph_index,
                                    flags_pos
                                )
                            })?;
                            let coordinate_len = match flags & 0x12 {
                                0x00 => 2,        // !X_SHORT_VECTOR | !X_IS_SAME
                                0x02 | 0x12 => 1, // X_SHORT_VECTOR
                                0x10 => 0,        // !X_SHORT_VECTOR | X_IS_SAME
                                _ => unreachable!(),
                            } + match flags & 0x24 {
                                0x00 => 2,        // !Y_SHORT_VECTOR | !Y_IS_SAME
                                0x04 | 0x24 => 1, // Y_SHORT_VECTOR
                                0x20 => 0,        // !Y_SHORT_VECTOR | Y_IS_SAME
                                _ => unreachable!(),
                            };
                            // REPEAT_FLAG
                            if flags & 0x08 != 0 {
                                flags_pos += 1;
                                let repeat_count = 1 + usize::try_from(
                                    *glyph.get(flags_pos).ok_or_else(|| {
                                        anyhow!(
                                            "sfnt {} table “glyf”: glyph {} data truncated ({})",
                                            sfnt_index,
                                            glyph_index,
                                            flags_pos
                                        )
                                    })?,
                                )
                                .unwrap();
                                flags_pos += 1;
                                glyph_len += coordinate_len * repeat_count;
                                i += repeat_count;
                                if i > num_of_points {
                                    bail!(
                                        "sfnt {} table “glyf”: glyph {} data truncated",
                                        sfnt_index,
                                        glyph_index
                                    );
                                }
                            } else {
                                flags_pos += 1;
                                glyph_len += coordinate_len;
                                i += 1;
                            }
                        }

                        glyf_modified = glyf_modified || instruction_len != 0;
                        let mut new_glyph = glyph
                            .get(..10 + num_of_contours * 2)
                            .ok_or_else(|| {
                                anyhow!(
                                    "sfnt {} table “glyf”: glyph {} data truncated (0..{})",
                                    sfnt_index,
                                    glyph_index,
                                    10 + num_of_contours * 2
                                )
                            })?
                            .to_vec();
                        new_glyph.extend_from_slice(&[0; 2]);
                        new_glyph.extend_from_slice(
                            glyph
                                .get(
                                    12 + num_of_contours * 2 + instruction_len
                                        ..flags_pos + glyph_len,
                                )
                                .ok_or_else(|| {
                                    anyhow!(
                                        "sfnt {} table “glyf”: glyph {} data truncated ({}..{})",
                                        sfnt_index,
                                        glyph_index,
                                        12 + num_of_contours * 2 + instruction_len,
                                        flags_pos + glyph_len
                                    )
                                })?,
                        );
                        new_glyph
                    } else {
                        // Composite glyph
                        let mut next_glyph = 0;
                        let mut glyph_len = 0;
                        let mut flags = [0; 2].as_ref();
                        let mut more_components = true;
                        while more_components {
                            next_glyph = glyph_len;
                            flags = glyph.get(next_glyph..next_glyph + 2).ok_or_else(|| {
                                anyhow!(
                                    "sfnt {} table “glyf”: glyph {} data truncated ({}..{})",
                                    sfnt_index,
                                    glyph_index,
                                    glyph_len,
                                    glyph_len + 2
                                )
                            })?;
                            glyph_len = next_glyph + 4;
                            if flags[1] & 0x01 != 0 {
                                // ARG_1_AND_2_ARE_WORDS
                                glyph_len += 4;
                            } else {
                                glyph_len += 2;
                            }
                            if flags[1] & 0x08 != 0 {
                                // WE_HAVE_A_SCALE
                                glyph_len += 2;
                            } else if flags[1] & 0x40 != 0 {
                                // WE_HAVE_AN_X_AND_Y_SCALE
                                glyph_len += 4;
                            } else if flags[1] & 0x80 != 0 {
                                // WE_HAVE_A_TWO_BY_TWO
                                glyph_len += 8;
                            }
                            // MORE_COMPONENTS
                            more_components = flags[1] & 0x20 != 0;
                        }

                        // WE_HAVE_INSTRUCTIONS
                        let we_have_instructions = flags[0] & 0x01 != 0;
                        glyf_modified = glyf_modified || we_have_instructions;
                        let mut new_glyph = glyph
                            .get(..next_glyph)
                            .ok_or_else(|| {
                                anyhow!(
                                    "sfnt {} table “glyf”: glyph {} data truncated (0..{})",
                                    sfnt_index,
                                    glyph_index,
                                    glyph_len
                                )
                            })?
                            .to_vec();
                        new_glyph.push(flags[0] & 0xfe);
                        new_glyph.push(flags[1]);
                        new_glyph.extend_from_slice(
                            glyph.get(next_glyph + 2..glyph_len).ok_or_else(|| {
                                anyhow!(
                                    "sfnt {} table “glyf”: glyph {} data truncated (0..{})",
                                    sfnt_index,
                                    glyph_index,
                                    glyph_len
                                )
                            })?,
                        );
                        new_glyph
                    };
                    Ok(Cow::Owned(new_glyph))
                })()
                .unwrap_or_else(|e: anyhow::Error| {
                    eprintln!("[ FAIL ] {}", e);
                    Cow::Borrowed(glyph)
                })
            })
            .collect::<Vec<_>>();

        if !glyf_modified {
            // "glyf" does not need modification, simply update "head" and skip.
            let head = sfnt.table_records.get_mut(&b"head".into()).unwrap();
            let mut new_head = head.raw_data.to_vec();
            // Byte 17: flags
            // flags[bit 2]: instructions may depend on point size
            // flags[bit 3]: force ppem to integer values
            // flags[bit 4]: instructions may alter advance width
            new_head[17] &= 0xf1;
            head.raw_data = Rc::from(new_head);
            continue;
        }
        eprintln!(
            "[ INFO ] sfnt {} table “glyf”: modified to remove per-glyph hinting.",
            sfnt_index
        );

        let mut new_raw_glyf =
            Vec::with_capacity(new_glyf.iter().map(|x| x.as_ref().len() + 1).sum());
        let mut new_loca = Vec::with_capacity(new_glyf.len() + 1);
        for glyf in new_glyf {
            if new_raw_glyf.len() % 2 != 0 {
                new_raw_glyf.push(0);
            }
            new_loca.push(new_raw_glyf.len());
            new_raw_glyf.extend(glyf.as_ref());
        }
        if new_raw_glyf.len() < 131072 && new_raw_glyf.len() % 2 != 0 {
            new_raw_glyf.push(0);
        }
        new_loca.push(new_raw_glyf.len());

        if new_raw_glyf.len() < 131072 {
            let mut new_raw_loca = Vec::with_capacity(new_loca.len() * 2);
            for x in new_loca {
                assert_eq!(x % 2, 0);
                new_raw_loca.extend_from_slice(&u16::try_from(x / 2).unwrap().to_be_bytes());
            }
            sfnt.table_records.insert(
                b"loca".into(),
                TableRecord {
                    checksum: 0,
                    offset: 0,
                    raw_data: Rc::from(new_raw_loca),
                },
            );
            sfnt.table_records.insert(
                b"glyf".into(),
                TableRecord {
                    checksum: 0,
                    offset: 0,
                    raw_data: Rc::from(new_raw_glyf),
                },
            );

            let head = sfnt.table_records.get_mut(&b"head".into()).unwrap();
            let mut new_head = head.raw_data.to_vec();
            // Byte 17: flags
            // flags[bit 2]: instructions may depend on point size
            // flags[bit 3]: force ppem to integer values
            // flags[bit 4]: instructions may alter advance width
            new_head[17] &= 0xf1;
            // byte 50..52: indexToLocFormat
            new_head[50..52].fill(0);
            head.raw_data = Rc::from(new_head);
        } else {
            let mut new_raw_loca = Vec::with_capacity(new_loca.len() * 4);
            for x in new_loca {
                if let Ok(x) = u32::try_from(x) {
                    new_raw_loca.extend_from_slice(&x.to_be_bytes());
                } else {
                    eprintln!(
                        "[ WARN ] sfnt {} table “glyf”: size exceeds 2 GiB",
                        sfnt_index
                    );
                    continue;
                }
            }
            sfnt.table_records.insert(
                b"loca".into(),
                TableRecord {
                    checksum: 0,
                    offset: 0,
                    raw_data: Rc::from(new_raw_loca),
                },
            );
            sfnt.table_records.insert(
                b"glyf".into(),
                TableRecord {
                    checksum: 0,
                    offset: 0,
                    raw_data: Rc::from(new_raw_glyf),
                },
            );

            let head = sfnt.table_records.get_mut(&b"head".into()).unwrap();
            let mut new_head = head.raw_data.to_vec();
            // Byte 17: flags
            // flags[bit 2]: instructions may depend on point size
            // flags[bit 3]: force ppem to integer values
            // flags[bit 4]: instructions may alter advance width
            new_head[17] &= 0xf1;
            // byte 50..52: indexToLocFormat
            new_head[50..52].clone_from_slice(&[0, 1]);
            head.raw_data = Rc::from(new_head);
        }
    }
}

pub fn regenerate_gasp(ttc: &mut TTCHeader) {
    const PATCHED_GASP: [u8; 8] = [
        0x00, 0x01, // version
        0x00, 0x01, // numRanges
        0xff, 0xff, // gaspRanges[0].rangeMaxPPEM = 65535
        0x00,
        0x0a, // gaspRanges[0].rangeGaspBehavior = GASP_DO_GRAY | GASP_SYMMETRIC_SMOOTHING
    ];

    for sfnt in ttc.table_directories.iter_mut() {
        // Grid-fitting/scan-conversion
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
            let mut raw_data_copy = head.raw_data.to_vec();

            // Byte 16: flags
            // flags[bit 11]: font data is lossless converted
            // flags[bit 13]: font optimized for Microsoft ClearType
            raw_data_copy.get_mut(16).map(|x| *x |= 0x28);
            // Byte 46..48: smallest readable size in pixels
            raw_data_copy.get_mut(46..48).map(|x| x.fill(0));

            head.raw_data = Rc::from(raw_data_copy);
        }
    }
}
