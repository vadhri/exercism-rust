extern crate regex;
use failure::Error;
use failure::format_err;

use regex::RegexBuilder;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,
    only_file_names_with_matches: bool,
    case_sensitive: bool,
    invert_search: bool,
    full_line_match: bool
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut flags_object: Flags = Flags {
            line_numbers: false,
            only_file_names_with_matches: false,
            case_sensitive: false,
            invert_search: false,
            full_line_match: false
        };

        for flag in flags {
            match flag {
                &"-n" => {
                    flags_object.line_numbers = true;
                },
                &"-l" => {
                    flags_object.only_file_names_with_matches = true;
                },
                &"-i" => {
                    flags_object.case_sensitive = true;
                },
                &"-v" => {
                    flags_object.invert_search = true;
                },
                &"-x" => {
                    flags_object.full_line_match = true;
                },
                _rest => {}
            }
        }

        flags_object
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut regex_string = String::new();

    let mut output: Vec<String> = Vec::new();
    let mut result: Result<Vec<String>, Error> =  Ok(output.clone());

    if files.len() == 0 {
        result = Err(format_err!("0"));
    }

    if flags.full_line_match {
        regex_string.push('^');
        regex_string.push_str(pattern);
        regex_string.push('$');
    } else {
        regex_string.push_str(pattern);
    }

    let mut re = RegexBuilder::new(&regex_string);

    if flags.case_sensitive {
        re.case_insensitive(true);
    }

    let regex = re.build().unwrap();

    let mut add_result = |line_string: String, index: u32, filename: String| {
        let mut temp_string = String::new();

        if files.len() > 1 {
            temp_string.push_str(&filename);
            temp_string.push(':');
        }

        if flags.line_numbers {
            temp_string.push_str(&(index + 1).to_string());
            temp_string.push(':');
        }

        temp_string.push_str(&line_string);

        if flags.only_file_names_with_matches {
            if !output.contains(&filename) {
                output.push(filename);
            }
        } else {
            output.push(temp_string.clone());
        }
    };

    for file in files {
        let filed = File::open(file)?;
        let reader = BufReader::new(filed);

        for (index, line) in reader.lines().enumerate() {
            let line_string = &line.unwrap();

            match regex.find(line_string) {
                Some(_value) => {
                    if !flags.invert_search {
                        add_result(line_string.to_string(), index as u32, file.to_string());
                    }
                },
                None => {
                    if flags.invert_search {
                        add_result(line_string.to_string(), index as u32, file.to_string());
                    }
                }
            }
        }
    }


    if result.is_err() {
        result
    } else {
        Ok(output)
    }
}
