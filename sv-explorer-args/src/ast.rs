use crate::lexer::{ArgumentError, Token};
use explorer_args::*;
use lalrpop_util::{ParseError, lalrpop_mod};
use logos::Logos;
lalrpop_mod!(pub explorer_args);

/// Parse a single line of the file list containing arguments
pub fn parse_line(code: &str) -> CommandResult {
    let lexer = Token::lexer(code);
    let parser = CommandParser::new();
    parser.parse(lexer)
}

/// Implemented commands that can be parsed from the arguments in a file list
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Define { define: String, arg: Option<String> },
    Include { directory: String },
    V { library_file: String },
    Y { library_dir: String },
    File { file: String },
}

pub type CommandParseError = ParseError<(), Token, ArgumentError>;
pub type CommandResult = Result<Command, CommandParseError>;

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
    fn parse_file() {
        let result = parse_line("../../path/to/file.sv").unwrap();
        assert_eq!(
            result,
            Command::File {
                file: "../../path/to/file.sv".to_string(),
            }
        );
    }
}
