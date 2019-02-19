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
    format!("lvl4 - dcgen unimplemented")
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

    }
}
