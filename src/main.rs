use std::env;
use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

use crate::arg_parser::ArgParser;

use self::ttc_reader::TTCReader;

extern crate anyhow;

mod arg_parser;
mod ttc_reader;
mod types;

fn main() -> Result<()> {
    let args = ArgParser::parse(env::args_os());

    let input_file = File::open(args.input_filename)?;
    let buffered_reader = BufReader::new(input_file);
    let ttc = TTCReader::new(buffered_reader).read_ttc()?;
    println!("{:#?}", ttc);

    Ok(())
}
