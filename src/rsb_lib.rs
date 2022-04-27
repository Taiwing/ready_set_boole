use std::convert::TryInto;
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
