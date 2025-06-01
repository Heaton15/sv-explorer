use std::fmt;

use logos::{Logos, SpannedIter};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ArgumentError {
    #[default]
    InvalidToken,
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"#.*\n?", error = ArgumentError)]
pub enum Token {
    #[token("+incdir+")]
    IncludeDir,
    #[token("+define+")]
    Define,
    #[token("-v")]
    V,
    #[token("-y")]
    Y,
    #[token("=")]
    Equals,
    #[token("\"")]
    Quotes,
    #[regex("[./_a-zA-Z][._0-9a-zA-Z/]*", |lex| lex.slice().to_string())]
    PathContent(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

// TODO: Reimplement the iterator for this lexer
impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, ArgumentError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn parse_define() {
        let mut lex = Token::lexer("+define+MY_NAME=true");
        assert_eq!(lex.next(), Some(Ok(Token::Define)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::PathContent("MY_NAME".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Equals)));
        assert_eq!(lex.next(), Some(Ok(Token::PathContent("true".to_string()))));
        assert_eq!(lex.next(), None)
    }

    #[test]
    fn parse_incdir() {
        let mut lex = Token::lexer("+incdir+../path/to/include_dir/");
        assert_eq!(lex.next(), Some(Ok(Token::IncludeDir)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::PathContent(
                "../path/to/include_dir/".to_string()
            )))
        );
        assert_eq!(lex.next(), None)
    }

    #[test]
    fn parse_empty_define() {
        let mut lex = Token::lexer("+define+RUN_GATE_SIMS");
        assert_eq!(lex.next(), Some(Ok(Token::Define)));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::PathContent("RUN_GATE_SIMS".to_string())))
        );
        assert_eq!(lex.next(), None)
    }

    #[test]
    fn parse_v() {
        let mut lex = Token::lexer("-v ./path/to/file.v");
        assert_eq!(lex.next(), Some(Ok(Token::V)));
        assert_eq!(lex.next(), Some(Ok(Token::PathContent("./path/to/file.v".to_string()))));
        assert_eq!(lex.next(), None)
    }

    #[test]
    fn parse_y() {
        let mut lex = Token::lexer("-y /path/to/dir");
        assert_eq!(lex.next(), Some(Ok(Token::Y)));
        assert_eq!(lex.next(), Some(Ok(Token::PathContent("/path/to/dir".to_string()))));
        assert_eq!(lex.next(), None)
    }

    #[test]
    fn parse_full_path_to_file() {
        let mut lex = Token::lexer("/path/to/file.v");
        assert_eq!(lex.next(), Some(Ok(Token::PathContent("/path/to/file.v".to_string()))));
        assert_eq!(lex.next(), None)
    }
}
