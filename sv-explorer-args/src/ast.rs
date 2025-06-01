use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

use crate::lexer::{ArgumentError, Token};
use explorer_args::*;
use lalrpop_util::{ParseError, lalrpop_mod};
use logos::Logos;
lalrpop_mod!(pub explorer_args);

pub type CommandParseError = ParseError<(), Token, ArgumentError>;
pub type CommandResult = Result<Command, CommandParseError>;

/// Implemented commands that can be parsed from the arguments in a file list
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Command {
    Define { define: String, arg: Option<String> },
    Include { directory: String },
    V { library_file: String },
    Y { library_dir: String },
    File { file: String },
}

/// Container of parsed defines
#[derive(Debug, PartialEq)]
pub struct DefineArgs {
    pub define: String,
    pub arg: Option<String>,
}

impl DefineArgs {
    fn new(define: String, arg: Option<String>) -> Self {
        Self {
            define: define,
            arg: arg,
        }
    }
}

/// Container of parsed include directories
#[derive(Debug, PartialEq)]
pub struct IncludeArgs {
    pub include_dir: String,
}

impl IncludeArgs {
    fn new(directory: String) -> Self {
        Self {
            include_dir: directory,
        }
    }
}

/// Container of parsed SV files
#[derive(Debug, PartialEq)]
pub struct FileArgs {
    pub file: String,
}

impl FileArgs {
    fn new(file: String) -> Self {
        Self { file: file }
    }
}

/// Container for all parsed types
pub struct SvParsedArgs {
    pub defines: Vec<DefineArgs>,
    pub includes: Vec<IncludeArgs>,
    pub files: Vec<FileArgs>,
}

impl SvParsedArgs {
    fn new() -> Self {
        Self {
            defines: Vec::new(),
            includes: Vec::new(),
            files: Vec::new(),
        }
    }
}

/// Parses a filelist.f and returns a database of files, defines, and includes that can be parsed
pub fn parse(filelist: &Path) -> SvParsedArgs {
    let lines = read_filelist(filelist)
        .unwrap_or_else(|e| panic!("Error<{}> Failed to read {:?}", e, filelist));

    let mut args = SvParsedArgs::new();

    lines.for_each(|l| match l {
        Ok(curr_line) => {
            let result = parse_line(&curr_line).unwrap();
            match result {
                Command::Define { define, arg } => args.defines.push(DefineArgs::new(define, arg)),
                Command::Include { directory } => args.includes.push(IncludeArgs::new(directory)),
                Command::V { .. } => (), // TODO: Implement how we compute files from V
                Command::Y { .. } => (), // TODO: Implement how we compute files from Y
                Command::File { file } => args.files.push(FileArgs::new(file)),
            };
        }
        Err(e) => panic!("Error<{}> Failed to read line from filelist.", e),
    });

    args
}

/// Parse a single line of the file list containing arguments
pub fn parse_line(code: &str) -> CommandResult {
    let lexer = Token::lexer(code);
    let parser = CommandParser::new();
    parser.parse(lexer)
}

/// Parses the input filelist into individual lines
fn read_filelist<P>(p: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let filelist = File::open(p)?;
    let filelist_lines = BufReader::new(filelist).lines();
    Ok(filelist_lines)
}

#[cfg(test)]
mod ast_tests {
    use super::*;
    #[test]
    fn parse_define() {
        let result = parse_line("+define+MY_NAME=true").unwrap();
        assert_eq!(
            result,
            Command::Define {
                define: "MY_NAME".to_string(),
                arg: Some("true".to_string())
            }
        );
    }
    #[test]
    fn parse_define_string_arg() {
        let result = parse_line("+define+MY_NAME=\"true\"").unwrap();
        assert_eq!(
            result,
            Command::Define {
                define: "MY_NAME".to_string(),
                arg: Some("true".to_string())
            }
        );
    }
    #[test]
    fn parse_define_no_arg() {
        let result = parse_line("+define+I_AM_DEFINED").unwrap();
        assert_eq!(
            result,
            Command::Define {
                define: "I_AM_DEFINED".to_string(),
                arg: None,
            }
        );
    }
    #[test]
    fn parse_incdir() {
        let result = parse_line("+incdir+../../path/to/dir/").unwrap();
        assert_eq!(
            result,
            Command::Include {
                directory: "../../path/to/dir/".to_string(),
            }
        );
    }
    #[test]
    fn parse_v() {
        let result = parse_line("-v ../../path/to/file.sv").unwrap();
        assert_eq!(
            result,
            Command::V {
                library_file: "../../path/to/file.sv".to_string(),
            }
        );
    }
    #[test]
    fn parse_y() {
        let result = parse_line("-y ../../path/to/dir/").unwrap();
        assert_eq!(
            result,
            Command::Y {
                library_dir: "../../path/to/dir/".to_string(),
            }
        );
    }
    #[test]
    fn parse_sv_file() {
        let result = parse_line("../../path/to/file.sv").unwrap();
        assert_eq!(
            result,
            Command::File {
                file: "../../path/to/file.sv".to_string(),
            }
        );
    }
}
