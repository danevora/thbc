use std::iter::Peekable;
use std::str::Chars;

/**
 * thbc - Tar Heel Basic Calculator
 *
 * Author: Daniel Evora
 * ONYEN: devora
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/**
 * The tokens types of `thbc` are defined below.
 */
#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown(char),
    Operator(char),
    Number(f64),
    Register(char),
    Assignment,
    LParen,
    RParen,
}

/**
 * The internal state of a Tokenizer is maintained by a peekable character
 * iterator over a &str's Chars.
 */
pub struct Tokenizer<'str> {
    chars: Peekable<Chars<'str>>,
}

impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

/**
 * The Iterator trait is implemented for Tokenizer. It will produce items of
 * type Token and has a `next` method that returns Option<Token>.
 */
impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    /**
     * The `next` method ignores leading whitespace and returns the next
     * complete Some(Token) in the Tokenizer's input string or None at all.
     */
    fn next(&mut self) -> Option<Token> {
        self.lex_whitespace();
        if let Some(c) = self.chars.peek() {
            Some(match c {
                '+' | '-' | '*' | '/' | '^' => self.lex_operator(),
                '=' => self.lex_assignment(),
                '0'...'9' => self.lex_number(),
                'a'...'z' => self.lex_register(),
                '(' | ')' => self.lex_paren(),
                _ => self.lex_unknown(),
            })
        } else {
            None
        }
    }
}

/*
 * Helper methods of Tokenizer are follow. None are defined as pub 
 * so these are internal methods only.
 */
impl<'str> Tokenizer<'str> {
    fn lex_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\n' => self.chars.next(),
                _ => break,
            };
        }
    }

    fn lex_register(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            'a'...'z' => Token::Register(c),
            _ => panic!("unknown register"),
        }
    }

    fn lex_assignment(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '=' => Token::Assignment,
            _ => panic!("Unexpected assignment helper"),
        }
    }

    fn lex_paren(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => panic!("unknown register"),
        }
    }

    fn lex_unknown(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        Token::Unknown(c)
    }

    fn lex_operator(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '=' | '+' | '-' | '*' | '/' | '^' => Token::Operator(c),
            _ => panic!("unknown operator"),
        }
    }

    fn lex_number(&mut self) -> Token {
        let mut s = String::new();
        self.lex_digits(&mut s);
        if let Some(c) = self.chars.peek() {
            if *c == '.' {
                self.chars.next();
                s.push('.');
                self.lex_digits(&mut s);
            }
        }
        Token::Number(s.parse::<f64>().unwrap())
    }

    fn lex_digits(&mut self, buffer: &mut String) {
        while let Some(c) = self.chars.peek() {
            match c {
                '0'...'9' => buffer.push(self.chars.next().unwrap()),
                _ => break,
            }
        }
    }
}
