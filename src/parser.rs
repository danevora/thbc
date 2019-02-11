use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

/**
 * thbc - Tar Heel Basic Calculator - Parser
 *
 * Author: <author>
 * ONYEN: <onyen>
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/* == Begin Syntax Tree Elements == */
#[derive(Debug, PartialEq)]
pub enum Expr {
    BinOp {
        lhs: Box<Expr>,
        op: char,
        rhs: Box<Expr>,
    },
    Num(f64),
}

/* Helper factory functions for building Exprs */
pub fn binop(lhs: Expr, op: char, rhs: Expr) -> Expr {
    Expr::BinOp {
        op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn num(value: f64) -> Expr {
    Expr::Num(value)
}
/* == End Syntax Tree Elements == */

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<Expr, String> {
        let mut parser = Parser {
            tokens: tokenizer.peekable(),
        };
        // TODO lvl0: Ensure no remaining tokens in parser after parsing Expr
        parser.expr()
    }
}

#[cfg(test)]
mod public_api {
    use super::*;

    mod lvl0 {
        use super::*;

        #[test]
        fn parse_atom_number() {
            let res = Parser::parse(Tokenizer::new("1")).unwrap();
            assert_eq!(num(1.0), res);
        }

        #[test]
        fn parse_atom_parens() {
            let res = Parser::parse(Tokenizer::new("(1)")).unwrap();
            assert_eq!(num(1.0), res);
        }

        #[test]
        fn parse_err_did_not_consume_whole_input() {
            let res = Parser::parse(Tokenizer::new("1 2"));
            assert_eq!(
                Err(String::from("Expected end of input, found Number(2.0)")),
                res
            );
        }
    }

    mod lvl1 {
        use super::*;

        #[test]
        fn parse_mul() {
            let res = Parser::parse(Tokenizer::new("1*2")).unwrap();
            assert_eq!(binop(num(1.0), '*', num(2.0)), res);
        }

        #[test]
        fn parse_div() {
            let res = Parser::parse(Tokenizer::new("1/2")).unwrap();
            assert_eq!(binop(num(1.0), '/', num(2.0)), res);
        }
    }

    mod lvl2 {
        use super::*;

        #[test]
        fn parse_div_chain() {
            let res = Parser::parse(Tokenizer::new("1/2/4")).unwrap();
            assert_eq!(binop(binop(num(1.0), '/', num(2.0)), '/', num(4.0)), res);
        }

        // TODO: add additional lvl2 tests
    }

    // TODO: Add tests for lvl > 0
}

/**
 * Internal-only parser methods to process the grammar via recursive descent.
 */
impl<'tokens> Parser<'tokens> {
    // Level 0
    // Expr     -> Atom
    fn expr(&mut self) -> Result<Expr, String> {
        Err(format!("lvl0 unimplemented"))
    }

    // Atom     -> '(' Expr ')' | Num
    fn atom(&mut self) -> Result<Expr, String> {
        Err(format!("lvl0 unimplemented"))
    }

    // Level 1:
    // MaybeMulDiv  -> Atom MulDivOp?
    fn maybe_mul_div(&mut self) -> Result<Expr, String> {
        Err(format!("lvl1 unimplemented"))
    }

    // MulDivOp     -> ('*'|'/') Atom
    /**
     * The lhs: Expr is passed in so that the syntax tree can grow "down" the lhs.
     */
    fn mul_div_op(&mut self, lhs: Expr) -> Result<Expr, String> {
        Err(format!("lvl1 unimplemented"))
    }

    // Level 2: Does not add new rules, rather modifies Level 1's!

    // Level 3:
    // MaybeAddSub -> MaybeMulDiv AddSubOp?
    // AddSubOp    -> ('+'|'-') MaybeMulDiv AddSubOp?
}

#[cfg(test)]
mod private_api {
    use super::*;

    mod lvl0 {
        use super::*;

        #[test]
        fn atom_ok() {
            assert_eq!(Parser::from("1").atom().unwrap(), num(1.0));
            assert_eq!(Parser::from("(1)").atom().unwrap(), num(1.0));
            assert_eq!(Parser::from("((1))").atom().unwrap(), num(1.0));
        }

        #[test]
        fn atom_err_empty_parens() {
            assert_eq!(
                Parser::from("()").atom(),
                Err(String::from("Unexpected token: RParen")),
            );
        }

        #[test]
        fn atom_err_not_an_atom() {
            assert_eq!(
                Parser::from("+").atom(),
                Err(String::from("Unexpected token: Operator('+')")),
            );
        }

        #[test]
        fn atom_err_incomplete() {
            assert_eq!(
                Parser::from("(").atom(),
                Err(String::from("Unexpected end of input"))
            );
            assert_eq!(
                Parser::from("(1").atom(),
                Err(String::from("Unexpected end of input"))
            );
        }
    }

    mod lvl1 {
        use super::*;

        #[test]
        fn maybe_mul_div_atom() {
            assert_eq!(Parser::from("1").maybe_mul_div().unwrap(), num(1.0));
        }

        #[test]
        fn maybe_mul_div() {
            assert_eq!(
                Parser::from("1*2").maybe_mul_div().unwrap(),
                binop(num(1.0), '*', num(2.0))
            );
            assert_eq!(
                Parser::from("1/2").maybe_mul_div().unwrap(),
                binop(num(1.0), '/', num(2.0))
            );
        }

        #[test]
        fn mul_div_op() {
            assert_eq!(
                Parser::from("*2").mul_div_op(num(1.0)).unwrap(),
                binop(num(1.0), '*', num(2.0))
            );
            assert_eq!(
                Parser::from("/2").mul_div_op(num(1.0)).unwrap(),
                binop(num(1.0), '/', num(2.0))
            );
        }
    }

    mod lvl2 {
        use super::*;

        #[test]
        fn maybe_mul_div_multiplication() {
            assert_eq!(
                Parser::from("1/2/3").maybe_mul_div().unwrap(),
                binop(binop(num(1.0), '/', num(2.0)), '/', num(3.0))
            );
        }

        #[test]
        fn mul_div_op_multiplication() {
            assert_eq!(
                Parser::from("*2*3").mul_div_op(num(1.0)).unwrap(),
                binop(binop(num(1.0), '*', num(2.0)), '*', num(3.0))
            );
            assert_eq!(
                Parser::from("*3")
                    .mul_div_op(binop(num(1.0), '*', num(2.0)))
                    .unwrap(),
                binop(binop(num(1.0), '*', num(2.0)), '*', num(3.0))
            );
        }
    }
}

/* Parser's Helper Methods to improve ergonomics of parsing */
impl<'tokens> Parser<'tokens> {
    /**
     * Static helper method used in unit tests to establish a
     * parser given a string.
     */
    fn from(input: &'tokens str) -> Parser<'tokens> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }

    /**
     * When you expect another token and want to take it directly
     * or raise an error that you expected another token here but
     * found the end of input. Example usage:
     *
     * let t: Token = self.take_next_token()?;
     *
     * Notice the ? usage will automatically propagate the Err or
     * unwrap the value of Ok.
     */
    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    /**
     * When you want to peek for an operator this helper method
     * will optionally return the operator's character value to you
     * or it will return None.
     */
    fn peek_operator(&mut self) -> Option<char> {
        if let Some(Token::Operator(op)) = self.tokens.peek() {
            Some(*op)
        } else {
            None
        }
    }

    /**
     * When you know you want to take an Operator token, this helper
     * method will optionally take it and return it or result in an
     * Err. Example usage:
     *
     * let op: Token = self.take_operator()?;
     */
    fn take_operator(&mut self) -> Result<char, String> {
        let token = self.tokens.next();
        if let Some(Token::Operator(op)) = token {
            Ok(op)
        } else {
            Err(format!("Expected operator, found {:?}", token))
        }
    }

    /**
     * When there's a specific token you expect next in the grammar
     * use this helper method. It will raise an Err if there is no
     * next token or if it is not _exactly_ the Token you expected
     * next. If it is the token you expected, it will return Ok(Token).
     */
    fn consume_token(&mut self, expected: Token) -> Result<Token, String> {
        if let Some(next) = self.tokens.next() {
            if next != expected {
                Err(format!("Expected: {:?} - Found {:?}", expected, next))
            } else {
                Ok(next)
            }
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }
}
