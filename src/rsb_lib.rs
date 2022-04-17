use std::convert::TryInto;
use std::collections::HashMap;
use std::io::{self, Write, BufWriter};
use std::fmt;

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

pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    if formula.len() == 0 {
        panic!("formula string is empty");
    }
    for op in formula.chars() {
        let right = if "01!".contains(op) { None } else { stack.pop() };
        let left = if "01".contains(op) { None } else { stack.pop() };
        match (op, left, right) {
            ('0', None, None) => stack.push(false),
            ('1', None, None) => stack.push(true),
            ('!', Some(a), None) => stack.push(!a),
            ('&', Some(a), Some(b)) => stack.push(a && b),
            ('|', Some(a), Some(b)) => stack.push(a || b),
            ('^', Some(a), Some(b)) => stack.push(a != b),
            ('>', Some(a), Some(b)) => stack.push(!a || b),
            ('=', Some(a), Some(b)) => stack.push(a == b),
            _ => panic!("'{}' is not a valid op or is missing an argument", op),
        }
    }
    if stack.len() > 1 {
        panic!("the stack should be empty at the end");
    }
    stack.pop().expect("nothing to return (stack is empty)")
}

fn write_truth<W: Write>(table: &mut String, writer: &mut Option<&mut BufWriter<W>>, truth: String) {
	if let Some(w) = writer {
		w.write(truth.as_bytes()).unwrap();
	} else {
		table.push_str(&truth)
	};
}

fn build_truth(varmap: &HashMap<char, bool>, keys: &Vec<char>, is_header: bool) -> String {
	let mut truth: String = String::with_capacity((27*4+1)*2);

    for key in keys.iter() {
        let value = *varmap.get(key).unwrap();
        let c = if is_header { *key } else if value { '1' } else { '0' };
        truth.push_str(&format!("| {} ", c));
    }
    truth.push_str("|");
    if is_header {
        let sep: String = truth.clone().chars()
            .map(|x| if x == '|' { '|' } else { '-' }).collect();
		format!("{}\n{}\n", truth, sep)
    } else { format!("{}\n", truth) }
}

fn find_truth(formula: &str, varmap: &mut HashMap<char, bool>) {
    let mut compiled_formula: String = String::with_capacity(formula.len());

    for c in formula.chars() {
        let op = match c {
            'A'..='Z' => { if *varmap.get(&c).unwrap() { '1' } else { '0' } },
            _ => c,
        };
        compiled_formula.push(op);
    }
    varmap.insert('=', eval_formula(&compiled_formula));
}

fn set_values(varmap: &mut HashMap<char, bool>, keys: &Vec<char>, values: u32) {
    let length: usize = keys.len() - 1;
    if values == 0 { return }
    for shift in 0..length {
        let value = (values >> shift) & 1 != 0;
        varmap.insert(keys[length - shift - 1], value);
    }
}

pub fn truth_table<W: Write>(formula: &str, mut writer: Option<&mut BufWriter<W>>) -> Option<String> {
    let mut varmap: HashMap<char, bool> = HashMap::with_capacity(27);
    let mut keys: Vec<char> = Vec::with_capacity(27);
	let mut table: String = String::new();
    let mut values: u32 = 0;

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                if varmap.insert(c, false) == None { keys.push(c); }
            },
            '0' | '1' => panic!("'{}' is not a valid op", c),
            _ => (),
        }
    }
    let values_max: u32 = 1 << keys.len();
    varmap.insert('=', false);
    keys.push('=');
    keys.sort();
    keys.rotate_left(1);
    write_truth(&mut table, &mut writer, build_truth(&varmap, &keys, true));
    loop {
        set_values(&mut varmap, &keys, values);
        find_truth(formula, &mut varmap);
        let truth = build_truth(&varmap, &keys, false);
		write_truth(&mut table, &mut writer, truth);
        values = values + 1;
        if values == values_max { break; }
    };
	Some(table)
}

pub fn print_truth_table(formula: &str) {
	let mut writer = BufWriter::new(io::stdout());
	truth_table(formula, Some(&mut writer));
}

#[derive(Debug, PartialEq)]
enum BooleanAstType {
	Variable,
	Negation,
	Conjunction,
	Disjunction,
	ExclusiveDisjunction,
	MaterialCondition,
	LogicalEquivalence,
}

impl fmt::Display for BooleanAstType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub struct BooleanAstNode {
	boolean_type: BooleanAstType,
	op_symbol: char,
	left: Option<Box<BooleanAstNode>>,
	right: Option<Box<BooleanAstNode>>,
}

impl fmt::Display for BooleanAstNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.ast_string())
	}
}

impl BooleanAstNode {
	fn new(c: char) -> BooleanAstNode {
		let boolean_type = match c {
			'A' ..='Z' => BooleanAstType::Variable,
			'!' => BooleanAstType::Negation,
			'&' => BooleanAstType::Conjunction,
			'|' => BooleanAstType::Disjunction,
			'^' => BooleanAstType::ExclusiveDisjunction,
			'>' => BooleanAstType::MaterialCondition,
			'=' => BooleanAstType::LogicalEquivalence,
			_ => panic!("'{}' is not a valid op", c),
		};
		BooleanAstNode { boolean_type, op_symbol: c, left: None, right: None }
	}

	fn init_child<T: Iterator<Item = char>>(&mut self, formula: &mut T) -> Box<BooleanAstNode> {
		if let Some(op) = formula.next() {
			let mut child = Box::new(BooleanAstNode::new(op));
			child.init_children(formula);
			child
		} else {
			panic!("missing operand for operator '{}'",
				self.boolean_type.to_string());
		}
	}

	fn init_children<T: Iterator<Item = char>>(&mut self, formula: &mut T) {
		match self.boolean_type {
			BooleanAstType::Variable => (),
			BooleanAstType::Negation => {
				self.left = Some(self.init_child(formula));
			},
			_ => {
				self.right = Some(self.init_child(formula));
				self.left = Some(self.init_child(formula));
			},
		}
	}

	pub fn build_ast_tree(formula: &str) -> BooleanAstNode {
		let mut iter = formula.chars().rev();
		let mut ast = if let Some(op) = iter.next() {
			BooleanAstNode::new(op)
		} else {
			panic!("formula string is empty");
		};
		ast.init_children(&mut iter);
		if let Some(_) = iter.next() {
			panic!("unused operands in formula string");
		}
		ast
	}

	fn has_left(&self) -> bool {
		match self.left {
			Some(_) => true,
			_ => false,
		}
	}

	fn has_right(&self) -> bool {
		match self.right {
			Some(_) => true,
			_ => false,
		}
	}

	fn node_string(mut tree: &mut String,
		node_opt: &Option<Box<BooleanAstNode>>, mut padding: String,
		pointer: &str, has_left_sibling: bool) {
		if let Some(node) = node_opt {
			tree.push_str(&format!("\n{}{}{}",
				padding, pointer, node.boolean_type.to_string()));
			if node.boolean_type == BooleanAstType::Variable {
				tree.push_str(&format!("({})", node.op_symbol));
			}
			padding.push_str(if has_left_sibling { "│  " } else { "   " });
			let pointer_left = "└──";
			let pointer_right = if node.has_left() { "├──" } else { "└──" };
			BooleanAstNode::node_string(&mut tree, &node.right, padding.clone(),
				pointer_right, node.has_left());
			BooleanAstNode::node_string(&mut tree, &node.left, padding,
				pointer_left, false);
		}
	}

	fn ast_string(&self) -> String {
		let mut tree = String::new();
		let pointer_left = "└──";
		let pointer_right = if self.has_left() { "├──" } else { "└──" };

		tree.push_str(&self.boolean_type.to_string());
		if self.boolean_type == BooleanAstType::Variable {
			tree.push_str(&format!("({})", self.op_symbol));
		}
		BooleanAstNode::node_string(&mut tree, &self.right, String::new(),
			pointer_right, self.has_left());
		BooleanAstNode::node_string(&mut tree, &self.left, String::new(),
			pointer_left, false);
		tree
	}
}

pub fn negation_normal_form(formula: &str) -> String {
	String::new()
}
