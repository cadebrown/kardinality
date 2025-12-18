use thiserror::Error;

use super::{BinOp, Call, Expr, LexError, Program, Span, Token, TokenKind, lex};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message} at {span:?}")]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl From<LexError> for ParseError {
    fn from(value: LexError) -> Self {
        Self {
            message: value.message,
            span: value.span,
        }
    }
}

pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    let tokens = lex(input)?;
    let mut p = Parser { tokens, i: 0 };
    p.parse_program()
}

struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut calls = Vec::new();

        self.consume_semi();
        while !self.at_eof() {
            calls.push(self.parse_call()?);
            self.consume_semi();
        }

        Ok(Program { calls })
    }

    fn parse_call(&mut self) -> Result<Call, ParseError> {
        let (name, name_span) = self.expect_ident()?;
        self.expect_simple(TokenKind::LParen)?;

        let mut args = Vec::new();
        if self.peek_is(&TokenKind::RParen) {
            let r = self.bump();
            let span = Span::merge(name_span, r.span);
            return Ok(Call {
                name,
                name_span,
                args,
                span,
            });
        }

        loop {
            args.push(self.parse_expr()?);
            if self.peek_is(&TokenKind::Comma) {
                self.bump();
                continue;
            }
            let r = self.expect_simple(TokenKind::RParen)?;
            let span = Span::merge(name_span, r.span);
            return Ok(Call {
                name,
                name_span,
                args,
                span,
            });
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_add()
    }

    fn parse_add(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_mul()?;
        while self.peek_is(&TokenKind::Plus) {
            self.bump();
            let rhs = self.parse_mul()?;
            let span = Span::merge(expr.span(), rhs.span());
            expr = Expr::Binary {
                op: BinOp::Add,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_mul(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        while self.peek_is(&TokenKind::Star) {
            self.bump();
            let rhs = self.parse_primary()?;
            let span = Span::merge(expr.span(), rhs.span());
            expr = Expr::Binary {
                op: BinOp::Mul,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let t = self.peek().clone();
        match t.kind {
            TokenKind::NumUnary(n) => {
                let tok = self.bump();
                Ok(Expr::Int(n as i64, tok.span))
            }
            TokenKind::NumDigit(n) => {
                let tok = self.bump();
                Ok(Expr::Int(n as i64, tok.span))
            }
            TokenKind::Ident(name) => {
                let tok = self.bump();
                Ok(Expr::Var(name, tok.span))
            }
            TokenKind::LParen => {
                let l = self.bump();
                let inner = self.parse_expr()?;
                let r = self.expect_simple(TokenKind::RParen)?;
                let span = Span::merge(l.span, r.span);
                Ok(Expr::Group(Box::new(inner), span))
            }
            _ => Err(ParseError {
                message: "expected expression".to_string(),
                span: t.span,
            }),
        }
    }

    fn expect_ident(&mut self) -> Result<(String, Span), ParseError> {
        let t = self.peek().clone();
        match &t.kind {
            TokenKind::Ident(name) => {
                self.bump();
                Ok((name.clone(), t.span))
            }
            _ => Err(ParseError {
                message: "expected identifier".to_string(),
                span: t.span,
            }),
        }
    }

    fn expect_simple(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let t = self.peek().clone();
        if std::mem::discriminant(&t.kind) == std::mem::discriminant(&kind) {
            Ok(self.bump())
        } else {
            Err(ParseError {
                message: format!("expected {kind:?}"),
                span: t.span,
            })
        }
    }

    fn consume_semi(&mut self) {
        while self.peek_is(&TokenKind::Semi) {
            self.bump();
        }
    }

    fn peek_is(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    fn at_eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.i]
    }

    fn bump(&mut self) -> Token {
        let t = self.tokens[self.i].clone();
        self.i = (self.i + 1).min(self.tokens.len().saturating_sub(1));
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_calls() {
        let p = parse_program("score(11); bank(3) dbl()").unwrap();
        assert_eq!(p.calls.len(), 3);
        assert_eq!(p.calls[0].name, "score");
        assert!(matches!(p.calls[0].args[0], Expr::Int(2, _)));
        assert!(matches!(p.calls[1].args[0], Expr::Int(3, _)));
        assert_eq!(p.calls[2].args.len(), 0);
    }

    #[test]
    fn parses_registers_and_ops() {
        let p = parse_program("score(len_deck*11+1)").unwrap();
        assert_eq!(p.calls.len(), 1);
        assert_eq!(p.calls[0].name, "score");
    }
}
