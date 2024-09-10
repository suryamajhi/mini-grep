use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Digit,
    AlphaNumeric,
    PositiveCharGroup(String),
    NegativeCharGroup(String),
    Char(char),
    StartAnchor,
    EndAnchor,
    OneOrMore(char),
    ZeroOrOne(char),
    Wildcard,
    Alternation(String, String),
}

#[derive(Debug)]
pub enum PatternErr {
    InvalidPattern(String),
}

impl Display for PatternErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternErr::InvalidPattern(message) => write!(f, "Invalid pattern: {message}"),
        }
    }
}

impl Error for PatternErr {}
