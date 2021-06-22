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
use std::env::ArgsOs;
use std::ffi::OsString;
use std::process;

pub struct ArgParser {
    pub input_filename: OsString,
    pub output_filename: OsString,
    pub keep_bitmap: bool,
    pub keep_gasp: bool,
    pub keep_hinting: bool,
}

impl ArgParser {
    pub fn parse(args: ArgsOs) -> Self {
        let mut args = args;
        let program_name = args.next();

        enum CurrentOption {
            End,
            Output,
        }
        let mut current_option = Option::<CurrentOption>::None;
        let mut input_filename = Option::<OsString>::None;
        let mut output_filename = Option::<OsString>::None;
        let mut keep_bitmap = false;
        let mut keep_gasp = false;
        let mut keep_hinting = false;

        for arg in args {
            match current_option {
                None => {
                    if arg == "--" {
                        current_option = Some(CurrentOption::End);
                    } else if arg == "--help" {
                        Self::print_help_and_exit(&program_name, 0);
                    } else if arg == "--keep-bitmap" {
                        keep_bitmap = true;
                    } else if arg == "--keep-gasp" {
                        keep_gasp = true;
                    } else if arg == "--keep-hinting" {
                        keep_hinting = true;
                    } else if arg == "--modify-gasp" {
                        keep_gasp = false;
                    } else if arg == "--remove-bitmap" {
                        keep_bitmap = false;
                    } else if arg == "--remove-hinting" {
                        keep_hinting = false;
                    } else if arg == "-o" || arg == "--output" {
                        current_option = Some(CurrentOption::Output);
                    } else {
                        if input_filename.is_some() {
                            Self::print_help_and_exit(&program_name, 1);
                        }
                        input_filename = Some(arg);
                    }
                }
                Some(CurrentOption::End) => {
                    if input_filename.is_some() {
                        Self::print_help_and_exit(&program_name, 1);
                    }
                    input_filename = Some(arg);
                }
                Some(CurrentOption::Output) => {
                    if output_filename.is_some() {
                        Self::print_help_and_exit(&program_name, 1);
                    }
                    output_filename = Some(arg);
                    current_option = None;
                }
            }
        }
        match current_option {
            None => (),
            Some(CurrentOption::End) => (),
            _ => Self::print_help_and_exit(&program_name, 1),
        }

        Self {
            input_filename: input_filename
                .unwrap_or_else(|| Self::print_help_and_exit(&program_name, 1)),
            output_filename: output_filename
                .unwrap_or_else(|| Self::print_help_and_exit(&program_name, 1)),
            keep_bitmap,
            keep_gasp,
            keep_hinting,
        }
    }

    fn print_help_and_exit(program_name: &Option<OsString>, exit_code: i32) -> ! {
        let program_name = program_name
            .as_ref()
            .map_or(Cow::Borrowed("./faithtype"), |s| s.to_string_lossy());

        println!();
        println!(
            "Usage: {} -o OUTPUT.<otf,ttc,ttf> INPUT.<otf,ttc,ttf>",
            program_name
        );
        println!();
        println!("Options:");
        println!("    --remove-bitmap     Remove embedded bitmap.                    [Default]");
        println!("    --keep-bitmap       Do not remove embedded bitmap.");
        println!();
        println!("    --modify-gasp       Modify “gasp” table to enable 6 × 5 super- [Default]");
        println!("                        sampled  anti-aliasing.  Consider  also");
        println!("                        removing  hinting  instructions  if  the");
        println!("                        rendering becomes buggy at certain sizes.");
        println!("    --keep-gasp         Keep the original “gasp” table.");
        println!();
        println!("    --remove-hinting    Remove TrueType hinting instructions.      [Default]");
        println!("    --keep-hinting      Do not remove TrueType hinting.");
        println!();
        process::exit(exit_code);
    }
}
