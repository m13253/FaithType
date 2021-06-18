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

extern crate anyhow;

mod arg_parser;
mod checksum;
mod modify;
mod ttc_reader;
mod ttc_writer;
mod types;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

use anyhow::Result;

use self::arg_parser::ArgParser;
use self::ttc_reader::TTCReader;
use self::ttc_writer::TTCWriter;

fn main() -> Result<()> {
    let args = ArgParser::parse(env::args_os());

    let mut ttc = {
        let mut input_file = BufReader::new(File::open(args.input_filename)?);
        TTCReader::new(&mut input_file).read_ttc()?
    };

    modify::remove_dsig(&mut ttc);
    if !args.keep_bitmap {
        modify::remove_bitmap(&mut ttc);
    }
    if !args.keep_hinting {
        modify::remove_hinting(&mut ttc);
    }
    if !args.keep_gasp {
        modify::regenerate_gasp(&mut ttc);
    }
    modify::patch_head(&mut ttc);

    {
        let mut output_file = BufWriter::new(File::create(args.output_filename)?);
        TTCWriter::new(&mut output_file).write_ttc(&ttc)
    }?;

    Ok(())
}
