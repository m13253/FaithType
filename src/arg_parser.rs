use std::borrow::Cow;
use std::env::ArgsOs;
use std::ffi::OsString;
use std::process;

pub struct ArgParser {
    pub input_filename: OsString,
    pub output_filename: OsString,
}

impl ArgParser {
    pub fn parse(args: ArgsOs) -> Self {
        let mut args = args;
        let program_name = args.next();

        enum CurrentOption {
            None,
            End,
            Output,
        }
        let mut current_option = CurrentOption::None;
        let mut input_filename = Option::<OsString>::None;
        let mut output_filename = Option::<OsString>::None;

        for arg in args {
            match current_option {
                CurrentOption::None => {
                    if arg == "--" {
                        current_option = CurrentOption::End;
                    } else if arg == "--help" {
                        Self::print_help_and_exit(&program_name, 0);
                    } else if arg == "-o" || arg == "--output" {
                        current_option = CurrentOption::Output;
                    } else {
                        if input_filename.is_some() {
                            Self::print_help_and_exit(&program_name, 1);
                        }
                        input_filename = Some(arg);
                    }
                }
                CurrentOption::End => {
                    if input_filename.is_some() {
                        Self::print_help_and_exit(&program_name, 1);
                    }
                    input_filename = Some(arg);
                }
                CurrentOption::Output => {
                    if output_filename.is_some() {
                        Self::print_help_and_exit(&program_name, 1);
                    }
                    output_filename = Some(arg);
                    current_option = CurrentOption::None;
                }
            }
        }
        match current_option {
            CurrentOption::None | CurrentOption::End => (),
            _ => Self::print_help_and_exit(&program_name, 1),
        }

        Self {
            input_filename: input_filename
                .unwrap_or_else(|| Self::print_help_and_exit(&program_name, 1)),
            output_filename: output_filename
                .unwrap_or_else(|| Self::print_help_and_exit(&program_name, 1)),
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
        process::exit(exit_code);
    }
}
