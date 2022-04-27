use std::io::{self, BufWriter};
use crate::utils::{ast, truth};

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
	n ^ (n >> 1)
}

fn reverse_gray_code(mut gray: u32) -> u32 {
	let mut mask = gray;

	while mask != 0 {
		mask >>= 1;
		gray ^= mask;
	}
	gray
}

pub fn print_truth_table(formula: &str) {
	let mut writer = BufWriter::new(io::stdout());
	truth::table(formula, Some(&mut writer));
}

pub fn negation_normal_form(formula: &str) -> String {
	let mut ast = ast::BoolNode::tree(formula);
	ast.to_nnf();
	ast.to_formula()
}

pub fn conjunctive_normal_form(formula: &str) -> String {
	let mut ast = ast::BoolNode::tree(formula);
	ast.to_cnf();
	ast.to_formula()
}

pub fn map(x: u16, y: u16) -> f64 {
	let mut bits: u64;
	let mut result: f64;
	let mut offset: u8 = 0;
	let mut interleaved: u32 = 0;

	for shift in 0..16 {
		let x_bit = (x as u32 >> shift) & 1;
		interleaved |= x_bit << (15 - shift) * 2;
		let y_bit = (y as u32 >> shift) & 1;
		interleaved |= y_bit << (15 - shift) * 2 + 1;
	}
	bits = (gray_code(interleaved) as u64) << 32;
	result = f64::from_bits(bits);
	while result.is_sign_negative() || result > 1.0 || result.is_nan() {
		bits >>= 1;
		offset += 1;
		bits &= !0x0f;
		bits |= offset as u64;
		result = f64::from_bits(bits);
	}
	result
}

pub fn reverse_map(n: f64) -> (u16, u16) {
	if n.is_sign_negative() || n > 1.0 || n.is_nan() {
		panic!("input number '{}' is out of range [0;1]", n);
	}
	let bits = n.to_bits();
	let offset = (bits & 0x0f) as u8;
	let gray = ((bits >> (32 - offset)) & u32::MAX as u64) as u32;
	let interleaved = reverse_gray_code(gray);
	let mut x: u16 = 0;
	let mut y: u16 = 0;
	for shift in 0..16 {
		let x_bit = ((interleaved >> (15 - shift) * 2) & 1) as u16;
		x |= x_bit << shift;
		let y_bit = ((interleaved >> (15 - shift) * 2 + 1) & 1) as u16;
		y |= y_bit << shift;
	}
	(x, y)
}
