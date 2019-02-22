use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

/**
 * thbc - Tar Heel Basic Calculator - Parser
 *
 * Author: Daniel Evora
 * ONYEN: devora
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
        let mut parse = parser.expr(); //calling this should consume all the tokens in the input, stores the result in 'parse'
        let next = parser.take_next_token(); //checks to see if there is another token after calling expr()
        match next {
            Ok(token) => {
                Err(format!("Expected end of input, found {:?}", token)) //if there is another token, throws an error
            },
            Err(e) => parse, //otherwise, returns the result of calling expr() on parser
        }
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
        
        #[test]
        fn parse_mul_chain() {
            let res = Parser::parse(Tokenizer::new("1*2*4")).unwrap();
            assert_eq!(binop(binop(num(1.0), '*', num(2.0)), '*', num(4.0)), res);
        }
        
        #[test]
        fn parse_mul_and_div() {
            let res = Parser::parse(Tokenizer::new("1/2*4")).unwrap();
            assert_eq!(binop(binop(num(1.0), '/', num(2.0)), '*', num(4.0)), res);
        }

        #[test]
        fn parse_with_parens() {
            let res = Parser::parse(Tokenizer::new("1*(2*4)")).unwrap();
            assert_eq!(binop(num(1.0), '*', binop(num(2.0), '*', num(4.0))), res);
        }

        #[test]
        fn parse_with_bunch_of_parens() {
            let res = Parser::parse(Tokenizer::new("(1*((2*3)*4))")).unwrap();
            assert_eq!(binop(num(1.0), '*', binop(binop(num(2.0), '*', num(3.0)), '*', num(4.0))), res);
        }

    }

    mod lvl3 {
    
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
        if let Some(token) = self.tokens.peek() { //looks to see if there is a token in the input
            self.maybe_add_sub() //if there is, jumps to maybe_add_sub
        } else {
            Err(format!("Unexpected end of input")) //throws an error because nothing was entered into input or new expr() from atom() is empty
        }
    }

    // Atom     -> '(' Expr ')' | Num
    fn atom(&mut self) -> Result<Expr, String> {
        let next = self.take_next_token(); //takes in the next token
        match next {
            Ok(Token::LParen) => {
                let expr = self.expr()?; //if the next token is a LParen, creates a new Expr
                let right_paren = self.consume_token(Token::RParen)?;
                Ok(expr) //returns the expr inside of the parenthesis
            },
            Ok(Token::Number(c)) => Ok(num(c)), //if its just a number it returns that number as the atom
            _ => Err(format!("Unexpected end of input")) //returns an error becomes something is missing
        }
    }

    // Level 1:
    // MaybeMulDiv  -> Atom MulDivOp?
    fn maybe_mul_div(&mut self) -> Result<Expr, String> {
        let lhs = self.atom()?; //takes in the lhs argument of the input
        let oper = self.peek_operator();
        if let Some(op) = oper { //looks to see if there is an operator
            match oper.unwrap() {
                '*'|'/' => self.mul_div_op(lhs), //jumps to mul_div_op if operator is * or /
                _ => Ok(lhs),
            }
        } else {
            Ok(lhs) //just returns the lhs if there is no operator
        }
    }

    // MulDivOp     -> ('*'|'/') Atom
    /**
     * The lhs: Expr is passed in so that the syntax tree can grow "down" the lhs.
     */
    fn mul_div_op(&mut self, lhs: Expr) -> Result<Expr, String> {
        let op = self.take_operator()?; //takes in the operator after lhs
        let rhs = self.atom()?; //calls atom to find the rhs
        let bin = binop(lhs, op, rhs); //creates a binop with lhs, op, and rhs
        if let Some(operator) = self.peek_operator() { //checks to see if there is another op after rhs
            self.mul_div_op(bin) //calls mull_div_op if this is the case
        } else {
            Ok(bin) //returns the binop if the input is over
        }
    }

    // Level 3:
    // MaybeAddSub -> MaybeMulDiv AddSubOp?
    // AddSubOp    -> ('+'|'-') MaybeMulDiv AddSubOp?
    
    fn maybe_add_sub(&mut self) -> Result<Expr, String> {
        let lhs = self.maybe_mul_div()?;
        let oper = self.peek_operator();
        if let Some(op) = oper {
            match oper.unwrap() {
                '+'|'-' => self.add_sub_op(lhs),
                _ => Ok(lhs),
            }
        } else {
            Ok(lhs)
        }
    }

    fn add_sub_op(&mut self, lhs: Expr) -> Result<Expr, String> {
        let op = self.take_operator()?;
        let rhs = self.maybe_mul_div()?;
        let bin = binop(lhs, op, rhs);
        if let Some(operator) = self.peek_operator() {
            self.add_sub_op(bin)
        } else {
            Ok(bin)
        }
    }

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
        fn maybe_mul_div_division() {
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

        #[test]
        fn maybe_mul_div_multiplication() {
            assert_eq!(
                Parser::from("1*2*3").maybe_mul_div().unwrap(),
                binop(binop(num(1.0), '*', num(2.0)), '*', num(3.0))
            );
        }

        #[test]
        fn mul_div_op_division() {
            assert_eq!(
                Parser::from("/2/3").mul_div_op(num(1.0)).unwrap(),
                binop(binop(num(1.0), '/', num(2.0)), '/', num(3.0))
            );
            assert_eq!(
                Parser::from("/3")
                    .mul_div_op(binop(num(1.0), '/', num(2.0)))
                    .unwrap(),
                binop(binop(num(1.0), '/', num(2.0)), '/', num(3.0))
            );
        }

        #[test]
        fn maybe_mul_div_atom() {
            assert_eq!(Parser::from("1").maybe_mul_div().unwrap(), num(1.0));
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
     * let op: char = self.take_operator()?;
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
