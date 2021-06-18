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
        let mut keep_hinting = true;

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
            None | Some(CurrentOption::End) => (),
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
        println!("    --remove-hinting    Remove TrueType hinting instructions.");
        println!("                        Due to the complexity, not all instructions");
        println!("                        are  removed.  Use  “ttfautohint --dehint”");
        println!("                        before using this tool to really remove all");
        println!("                        hinting instructions.");
        println!("    --keep-hinting      Do not remove TrueType hinting.            [Default]");
        println!();
        process::exit(exit_code);
    }
}
