use crate::lexer::{ArgumentError, Token};
use explorer_args::*;
use lalrpop_util::{ParseError, lalrpop_mod};
use logos::Logos;
lalrpop_mod!(pub explorer_args);

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Define { define: String, arg: Option<String> },
    Include { directory: String },
}

pub type CommandParseError = ParseError<(), Token, ArgumentError>;
pub type CommandResult = Result<Command, CommandParseError>;

pub fn parse(code: &str) -> CommandResult {
    let lexer = Token::lexer(code);
    let parser = CommandParser::new();
    parser.parse(lexer)
}

#[cfg(test)]
mod ast_tests {
    use super::*;
    #[test]
    fn parse_define() {
        let result = parse("+define+MY_NAME=true").unwrap();
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
        let result = parse("+define+I_AM_DEFINED").unwrap();
        assert_eq!(
            result,
            Command::Define {
                define: "I_AM_DEFINED".to_string(),
                arg: None,
            }
        );
    }
    #[test]
    fn parse_include() {
        let result = parse("+include+../../path/to/dir/").unwrap();
        assert_eq!(
            result,
            Command::Include {
                directory: "../../path/to/dir".to_string(),
            }
        );
    }
}
