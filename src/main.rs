//! Simple Reverse Polish Notation calculator.
use std::env;

/// A LIFO stack of f64's.
type Stack = Vec<f64>;

/// Apply a binary operation to the stack.
fn binop<F>(stack: &mut Stack, f: F) -> f64
    where F: Fn(f64, f64) -> f64 {

    if let (Some(x), Some(y)) = (stack.pop(), stack.pop()) {
        f(x, y)
    } else {
        panic!("Not enough values on the stack.")
    }
}

/// Apply a unary operation to the stack.
fn unop<F>(stack: &mut Stack, f: F) -> f64
    where F: Fn(f64) -> f64 {

    if let Some(x) = stack.pop() {
        f(x)
    } else {
        panic!("Not enough values on the stack.")
    }
}

/// Calculate an arithmetic problem in Reverse Polish Notation.
fn rpn(glyphs: Vec<String>) -> f64 {
    let mut stack: Stack = Vec::new();

    for glyph in glyphs.into_iter() {
        match glyph.as_str() {
            // Addition.
            "+" => {
                let val = binop(&mut stack, |x, y| x + y);
                stack.push(val);
            },
            // Subtraction.
            "-" => {
                let val = binop(&mut stack, |x, y| x - y);
                stack.push(val);
            }
            // Multiplication.
            "*" | "x" => {
                let val = binop(&mut stack, |x, y| x * y);
                stack.push(val);
            },
            // Division.
            "/" => {
                let val = binop(&mut stack, |x, y| x / y);
                stack.push(val);
            },

            // Exponentation.
            "^" => {
                let val = binop(&mut stack, |x, y| {
                    let exp = y as u64;
                    if exp < 2 {
                        panic!("Exponents < 2 are not supported.");
                    }

                    let mut pow = x;
                    for _ in 1..exp {
                        pow *= x;
                    }
                    pow 
                });
                stack.push(val);
            },

            // Absolute value.
            "abs" => {
                let val = unop(&mut stack, |x| x.abs());
                stack.push(val);
            },

            // It's a number, so parse it and add it to the stack.
            n   => match n.parse::<f64>() {
                Ok(n) => stack.push(n),
                Err(_) => panic!("Invalid glyph: {}", n),
            },
        }
    }

    if stack.len() == 0 {
        panic!("Error: no values were given.");
    } else if stack.len() > 1 {
        println!("Warning: too many values on the stack");
    }

    stack[stack.len() - 1]
}

fn main() {
    let glyphs: Vec<String> = env::args().skip(1).collect();
    let result = rpn(glyphs);
    println!("{}", result);
}
