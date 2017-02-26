type Stack = Vec<f64>;

fn binop<F>(stack: &mut Stack, f: F) -> f64
    where F: Fn(f64, f64) -> f64 {

    if let (Some(x), Some(y)) = (stack.pop(), stack.pop()) {
        f(x, y)
    } else {
        panic!("Not enough values on the stack.")
    }
}

fn unop<F>(stack: &mut Stack, f: F) -> f64
    where F: Fn(f64) -> f64 {

    if let Some(x) = stack.pop() {
        f(x)
    } else {
        panic!("Not enough values on the stack.")
    }
}

fn rpn(glyphs: Vec<&str>) -> f64 {
    let mut stack: Stack = Vec::new();

    for glyph in glyphs.into_iter() {
        match glyph {
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
            "*" => {
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

    stack[0]
}

fn main() {
    let result = rpn(vec!["5", "3", "+", "3.5", "*", "56", "/", "3", "^"]);
    println!("Result: {}", result);
}
