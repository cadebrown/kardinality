use thiserror::Error;

use super::{Span, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message} at {span:?}")]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < input.len() {
        let c = input[i..].chars().next().unwrap();
        let c_len = c.len_utf8();

        if c.is_whitespace() {
            i += c_len;
            continue;
        }

        let start = i;

        let kind = match c {
            ';' => {
                i += 1;
                TokenKind::Semi
            }
            '(' => {
                i += 1;
                TokenKind::LParen
            }
            ')' => {
                i += 1;
                TokenKind::RParen
            }
            ',' => {
                i += 1;
                TokenKind::Comma
            }
            '+' => {
                i += 1;
                TokenKind::Plus
            }
            '*' => {
                i += 1;
                TokenKind::Star
            }
            '1' => {
                let mut count = 0u32;
                while i < input.len() && input[i..].starts_with('1') {
                    i += 1;
                    count += 1;
                }
                TokenKind::NumUnary(count)
            }
            '0'..='9' => {
                i += 1;
                TokenKind::NumDigit(c.to_digit(10).unwrap_or(0))
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                i += c_len;
                while i < input.len() {
                    let nc = input[i..].chars().next().unwrap();
                    if nc.is_ascii_alphanumeric() || nc == '_' {
                        i += nc.len_utf8();
                    } else {
                        break;
                    }
                }
                TokenKind::Ident(input[start..i].to_string())
            }
            _ => {
                return Err(LexError {
                    message: format!("unexpected character '{c}'"),
                    span: Span::new(start, start + c_len),
                });
            }
        };

        let end = i;
        out.push(Token {
            kind,
            span: Span::new(start, end),
        });
    }

    out.push(Token {
        kind: TokenKind::Eof,
        span: Span::new(input.len(), input.len()),
    });

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_calls_and_ops() {
        let t = lex("score(11+3); dbl()").unwrap();
        assert!(t.iter().any(|t| matches!(t.kind, TokenKind::Plus)));
        assert!(t.iter().any(|t| matches!(t.kind, TokenKind::Semi)));
    }
}
