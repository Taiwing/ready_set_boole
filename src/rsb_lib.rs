use std::convert::TryInto;

pub fn adder(mut a: u32, mut b: u32) -> u32 {
    let mut newbit: u32;
    let mut carry: u32 = 0;
    let mut result: u32 = 0;

    for _ in 0..32 {
        newbit = (carry ^ a ^ b) & 1;
        carry = if (a ^ b) & 1 != 0 { newbit ^ 1 } else { a & b & 1 };
        result = (result >> 1) | (newbit << 31);
        a = a >> 1;
        b = b >> 1;
    }
    result
}

pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
    let mut result: u32 = 0;

    while a != 0 {
        if (a & 1) != 0 {
            result = adder(result, b);
        }
        b = b << 1;
        a = a >> 1;
    }
    result
}

pub fn gray_code(n: u32) -> u32 {
    let tmp: u64 = n.into();

    (tmp ^ (tmp >> 1)).try_into().unwrap()
}

//TODO: fetch_args() function
pub fn eval_formula(formula: &str) -> bool {
    let stack: Vec<bool> = Vec::new();

    if formula.len() == 0 {
        panic!("formula string is empty");
    }
    for c in formula.chars() {
        match c {
            '0' => { stack.push(false); },
            '1' => { stack.push(true); },
            '!' => {
                if let Some(a) = stack.pop() {
                    stack.push(!a);
                } else {
                    panic!("operation '{}' requires an operand", c);
                }
            },
            '&' | '|' | '^' | '>' | '=' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    if c == '&' {
                        stack.push(a && b);
                    } else if c == '|' {
                        stack.push(a || b);
                    } else if c == '^' {
                        stack.push(a != b);
                    } else if c == '>' {
                        stack.push(a && b); //TODO
                    } else if c == '=' {
                        stack.push(a == b);
                    }
                } else {
                    panic!("operation '{}' requires two operands", c);
                }
            },
            _ => { panic!("'{}' is not a valid operand, must be one of: \"01!&|^>=\""); },
        }
    }
    stack.pop().unwrap()
}
