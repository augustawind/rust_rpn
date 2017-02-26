type Stack = Vec<f64>;

fn binop<F>(stack: &mut Stack, f: F) -> f64
    where F: Fn(f64, f64) -> f64 {

    if let (Some(x), Some(y)) = (stack.pop(), stack.pop()) {
        f(x, y)
    } else {
        panic!("Not enough values on the stack.")
    }
}

fn rpn(glyphs: Vec<&str>) -> f64 {
    let mut stack: Stack = Vec::new();

    for glyph in glyphs.into_iter() {
        match glyph {
            "+" => {
                let val = binop(&mut stack, |x, y| x + y);
                stack.push(val);
            },
            "-" => {
                let val = binop(&mut stack, |x, y| x - y);
                stack.push(val);
            }
            "*" => {
                let val = binop(&mut stack, |x, y| x * y);
                stack.push(val);
            },
            "/" => {
                let val = binop(&mut stack, |x, y| x / y);
                stack.push(val);
            },
            n   => match n.parse::<f64>() {
                Ok(n) => stack.push(n),
                Err(_) => panic!("Invalid glyph: {}", n),
            },
        }
    }

    stack[0]
}

fn main() {
    let result = rpn(vec!["5", "3", "+", "3.5", "-", "2", "*", "27", "/"]);
    println!("Result: {}", result);
}
