use super::parser::Expr;
use super::parser::{binop, num};

/**
 * thbc - Tar Heel Basic Calculator - DCGen
 *
 * Author: Daniel Evora
 * ONYEN: devora
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/**
 * Given a parser::Expr, to_dc should return a string representing the tree
 * in valid `dc` format with a `p` at the end. Each number and operator should
 * be separated by a space with no trailing space at the end after the `p`.
 *
 * You are encouraged to use helper functions and recursion where sensible.
 */
pub fn to_dc(expr: &Expr) -> String {
    let mut output: String = recur_to_dc(expr); //creates a string to return, calls helper since this would be hard to do recursivley in one function
    output.push('p'); //adds the print onto the end
    output //returns string
}

fn recur_to_dc(expr: &Expr) -> String {
    let mut output: String = String::new(); //creates a new string
    match expr { //sees if the expr is a binop or number
        Expr::BinOp{lhs, op, rhs} => {
            output.push_str(&recur_to_dc(lhs)); //moves down recursively until a number is found
            output.push_str(&recur_to_dc(rhs)); //same but for the right side
            output.push(*op); //adds the operator
            output.push(' '); //adds a space
        },
        Expr::Num(num) => {
            output.push_str(&num.to_string()); //adds the number to output
            output.push(' '); //adds a space after each number
        },
    }
    output
}

#[cfg(test)]
mod to_dc {
    use super::*;

    mod lvl4 {
        use super::*;

        #[test]
        fn dc_num() {
            assert_eq!("1 p", to_dc(&num(1.0)));
        }

        #[test]
        fn dc_binop() {
            assert_eq!("1 2 * p", to_dc(&binop(num(1.0), '*', num(2.0))));
        }

        // TODO: Add additional tests
        
        #[test]
        fn dc_multiple_ops() {
            assert_eq!(
                "1 1 / 1 - p",
                to_dc(&binop(binop(num(1.0), '/', num(1.0)), '-', num(1.0)))
            );
        }
        
    }
}
