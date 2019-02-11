#![allow(unused)]
// TODO: Remove the line above before submission and fix warnings!

/**
 * thbc - Tar Heel Basic Calculator
 *
 * Author: <author>
 * ONYEN: <onyen>
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/**
 * thbc - Tar Heel Basic Calculator
 *
 * This program begins to implement the essence of the `bc` utility.
 */
extern crate structopt;

const QUIT_STRING: &str = "quit\n";
const EXIT_OK: i32 = 0;
const EXIT_ERR: i32 = 1;

use std::io;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "thbc", about = "Tar Heel Basic Calculator")]
struct Options {
    #[structopt(short = "t", long = "show-tokens")]
    show_tokens: bool,
    #[structopt(short = "p", long = "show-parse")]
    show_parse: bool,
}

pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;
pub mod dc_gen;

fn main() {
    let options = Options::from_args();
    loop {
        eval(&read(), &options);
    }
}

fn eval(input: &str, options: &Options) {
    if options.show_tokens {
        eval_show_tokens(input);
    }

    if options.show_parse {
        eval_show_parse(input);
    }

    eval_target(input);
}

fn eval_show_tokens(input: &str) {
    println!("== Tokens ==");
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}

fn eval_show_parse(input: &str) {
    println!("== Parse Tree ==");
    match Parser::parse(Tokenizer::new(input)) {
        Ok(statement) => {
            println!("{:?}", statement);
        }
        Err(msg) => eprintln!("thbc: {}", msg),
    }
    print!("\n");
}

fn eval_target(input: &str) {
    match Parser::parse(Tokenizer::new(input)) {
        Ok(statement) => {
            println!("{}", dc_gen::to_dc(&statement));
        }
        Err(msg) => eprintln!("thbc: {}", msg),
    }
}

/**
 * Read input from the user. We'll handle the case of quitting
 * via the string "quit" and exit the program from here.
 */
fn read() -> String {
    match read_line() {
        Ok(line) => {
            if line == QUIT_STRING {
                // Exit the process with an Ok exit code.
                std::process::exit(EXIT_OK);
            } else {
                line
            }
        }
        Err(message) => {
            eprintln!("Err: {}", message);
            std::process::exit(EXIT_ERR);
        }
    }
}

/**
 * Helper function to read a line of input from stdin.
 */
fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
