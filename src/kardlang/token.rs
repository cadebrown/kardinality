use serde::{Deserialize, Serialize};

use super::Span;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenKind {
    Ident(String),
    NumUnary(u32),
    NumDigit(u32),
    Plus,
    Star,
    LParen,
    RParen,
    Comma,
    Semi,
    Eof,
}
