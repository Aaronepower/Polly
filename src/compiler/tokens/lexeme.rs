use std::fmt;
use std::fmt::{Display, Formatter};

use self::Lexeme::*;
use super::operator::Operator;

/// Parent enum defining the two types of Terminal symbols within the language.
/// Words, and operator symbols.
#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    /// TODO
    Op(usize, Operator),
    /// TODO
    Word(usize, String),
    Empty,
}

impl Lexeme {
    pub fn length(&self) -> usize {
        match *self {
            Op(_, _) => 1,
            Word(_, ref word) => word.len(),
            Empty => 0,
        }
    }

    pub fn index(&self) -> usize {
        match *self {
            Op(index, _) => index,
            Word(index, _) => index,
            Empty => unreachable!(),
        }
    }
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let display = match *self {
            Op(_, ref operator) => format!("OPERATOR: {}", operator.to_string()),
            Word(_, ref word) => format!("WORD: {}", word.clone()),
            Empty => unreachable!(),
        };

        write!(f, "{}", display)
    }
}