use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

use anyhow::Result;

use crate::ttc_writer::TTCWriter;

use self::arg_parser::ArgParser;
use self::ttc_reader::TTCReader;

extern crate anyhow;

mod arg_parser;
mod checksum;
mod ttc_reader;
mod ttc_writer;
mod types;

fn main() -> Result<()> {
    let args = ArgParser::parse(env::args_os());

    let ttc = {
        let mut input_file = BufReader::new(File::open(args.input_filename)?);
        TTCReader::new(&mut input_file).read_ttc()?
    };

    println!("{:?}", ttc);

    {
        let mut output_file = BufWriter::new(File::create(args.output_filename)?);
        TTCWriter::new(&mut output_file).write_ttc(&ttc)
    }?;

    Ok(())
}
