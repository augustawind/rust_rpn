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
        let val = match glyph.as_str() {
            // Addition.
            "+"         => binop(&mut stack, |x, y| x + y),
            // Subtraction.
            "-"         => binop(&mut stack, |x, y| x - y),
            // Multiplication.
            "*" | "x"   => binop(&mut stack, |x, y| x * y),
            // Division.
            "/"         => binop(&mut stack, |x, y| x / y),

            // Absolute value.
            "abs"       => unop(&mut stack, |x| x.abs()),

            // Exponentation.
            "^" => {
                binop(&mut stack, |x, y| {
                    let exp = y as u64;
                    if exp < 2 {
                        panic!("Exponents < 2 are not supported.");
                    }

                    let mut pow = x;
                    for _ in 1..exp {
                        pow *= x;
                    }
                    pow 
                })
            },

            // It's a number, so parse it and add it to the stack.
            n => match n.parse::<f64>() {
                Ok(n) => n,
                Err(_) => panic!("Invalid glyph: {}", n),
            },
        };

        stack.push(val);
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
