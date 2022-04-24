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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BooleanAstType {
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

#[derive(Debug)]
pub struct BooleanAstNode {
	boolean_type: BooleanAstType,
	op_symbol: char,
	left: Option<Box<Self>>,
	right: Option<Box<Self>>,
}

impl fmt::Display for BooleanAstNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.tree_string())
	}
}

impl Clone for BooleanAstNode {
	fn clone(&self) -> Self {
		let mut ast = Self::new(self.op_symbol);
		if let Some(left_node) = &self.left {
			ast.left = Some(Box::new(*left_node.clone()));
		}
		if let Some(right_node) = &self.right {
			ast.right = Some(Box::new(*right_node.clone()));
		}
		ast
	}
}

impl PartialEq for BooleanAstNode {
	/// This is not the smartest way to do this, but it was already done lol
	fn eq(&self, other: &Self) -> bool {
		let left_formula = self.to_formula();
		let right_formula = other.to_formula();
		if left_formula == right_formula { return true };
		match (truth_table::<std::io::Stdout>(&left_formula, None),
			truth_table::<std::io::Stdout>(&right_formula, None)) {
			(Some(left_truth), Some(right_truth)) => {
				left_truth == right_truth
			},
			_ => panic!("missing truth"),
		}
	}
}

impl Eq for BooleanAstNode {}

impl BooleanAstNode {
	fn symbol_to_type(c: char) -> BooleanAstType {
		match c {
			'A' ..='Z' => BooleanAstType::Variable,
			'!' => BooleanAstType::Negation,
			'&' => BooleanAstType::Conjunction,
			'|' => BooleanAstType::Disjunction,
			'^' => BooleanAstType::ExclusiveDisjunction,
			'>' => BooleanAstType::MaterialCondition,
			'=' => BooleanAstType::LogicalEquivalence,
			_ => panic!("'{}' is not a valid op", c),
		}
	}

	fn type_to_symbol(boolean_type: BooleanAstType) -> char {
		match boolean_type {
			BooleanAstType::Negation => '!',
			BooleanAstType::Conjunction => '&',
			BooleanAstType::Disjunction => '|',
			BooleanAstType::ExclusiveDisjunction => '^',
			BooleanAstType::MaterialCondition => '>',
			BooleanAstType::LogicalEquivalence => '=',
			BooleanAstType::Variable => {
				panic!("no predefined symbol for '{}' type", boolean_type);
			},
		}
	}

	fn change_type(&mut self, new_type: BooleanAstType) {
		self.boolean_type = new_type;
		self.op_symbol = Self::type_to_symbol(new_type);
	}

	fn new(c: char) -> Self {
		let boolean_type = Self::symbol_to_type(c);
		Self { boolean_type, op_symbol: c, left: None, right: None }
	}

	fn init_child<T: Iterator<Item = char>>(&mut self, formula: &mut T) -> Box<Self> {
		if let Some(op) = formula.next() {
			let mut child = Box::new(Self::new(op));
			child.init_children(formula);
			child
		} else {
			panic!("missing operand for '{}' operation", self.boolean_type);
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

	pub fn tree(formula: &str) -> Self {
		let mut iter = formula.chars().rev();
		let mut ast = if let Some(op) = iter.next() {
			Self::new(op)
		} else {
			panic!("formula string is empty");
		};
		ast.init_children(&mut iter);
		if let Some(_) = iter.next() {
			panic!("unused operands in formula string");
		}
		ast
	}

	pub fn pre_order(&mut self, op: impl Fn(&mut Self) + Copy) {
		op(self);
		if let Some(left_node) = &mut self.left {
			left_node.pre_order(op);
		}
		if let Some(right_node) = &mut self.right {
			right_node.pre_order(op);
		}
	}

	pub fn in_order(&mut self, op: impl Fn(&mut Self) + Copy) {
		if let Some(left_node) = &mut self.left {
			left_node.in_order(op);
		}
		op(self);
		if let Some(right_node) = &mut self.right {
			right_node.in_order(op);
		}
	}

	pub fn post_order(&mut self, op: impl Fn(&mut Self) + Copy) {
		if let Some(left_node) = &mut self.left {
			left_node.post_order(op);
		}
		if let Some(right_node) = &mut self.right {
			right_node.post_order(op);
		}
		op(self);
	}

	pub fn side_order<F: Fn(&mut Self) + Copy>(&mut self, op: F,
		order: impl Fn(&mut Self, F) + Copy, is_left: bool) {
		if let Some(left_node) = &mut self.left {
			if is_left {
				order(left_node, op);
			} else {
				left_node.side_order(op, order, is_left);
			}
		}
		if let Some(right_node) = &mut self.right {
			if is_left == false {
				order(right_node, op);
			} else {
				right_node.side_order(op, order, is_left);
			}
		}
	}

	pub fn is_valid_rotation(&self, boolean_type: BooleanAstType, is_left: bool) -> bool {
		match boolean_type {
			BooleanAstType::Variable => {
				panic!("cannot rotate '{}' node because it has no operand",
					boolean_type);
			},
			BooleanAstType::Negation => {
				panic!("cannot rotate '{}' node because it only has 1 operand",
					boolean_type);
			},
			BooleanAstType::MaterialCondition => {
				panic!("cannot rotate '{}' node because it is non-commutative",
					boolean_type);
			},
			_ => (),
		}
		if self.boolean_type != boolean_type { return false };
		match (&self.left, &self.right, is_left) {
			(Some(left), _, false) => {
				left.boolean_type == boolean_type
			},
			(_, Some(right), true) => {
				right.boolean_type == boolean_type
			},
			_ => panic!("invalid '{}' op", self.boolean_type),
		}
	}

	pub fn right_rotate(&mut self, boolean_type: BooleanAstType) {
		if self.is_valid_rotation(boolean_type, false) == false { return };
		std::mem::swap(&mut self.left, &mut self.right);
		if let Some(right) = &mut self.right {
			std::mem::swap(&mut right.left, &mut right.right);
			std::mem::swap(&mut self.left, &mut right.right);
			std::mem::swap(&mut self.op_symbol, &mut right.op_symbol);
			std::mem::swap(&mut self.boolean_type, &mut right.boolean_type);
		}
	}

	pub fn left_rotate(&mut self, boolean_type: BooleanAstType) {
		if self.is_valid_rotation(boolean_type, true) == false { return };
		std::mem::swap(&mut self.left, &mut self.right);
		if let Some(left) = &mut self.left {
			std::mem::swap(&mut left.left, &mut left.right);
			std::mem::swap(&mut self.right, &mut left.left);
			std::mem::swap(&mut self.op_symbol, &mut left.op_symbol);
			std::mem::swap(&mut self.boolean_type, &mut left.boolean_type);
		}
	}

	pub fn consume(
		mut self
	) -> (BooleanAstType, char, Option<Box<Self>>, Option<Box<Self>>) {
		(self.boolean_type, self.op_symbol, self.left, self.right)
	}

	fn get_operands(
		&mut self,
		target_type: BooleanAstType,
	) -> (Vec<Option<Box<Self>>>, Vec<Option<Box<Self>>>) {
		let mut node: Option<Box<Self>>;
		let mut left_operands: Vec<Option<Box<Self>>> = vec![];
		let mut right_operands: Vec<Option<Box<Self>>> = vec![];

		match (&mut self.left, &mut self.right) {
			(Some(l), Some(r)) => {
				if self.boolean_type != target_type
					&& l.boolean_type != target_type
					&& r.boolean_type != target_type {
					return (left_operands, right_operands);
				}
				if l.boolean_type != target_type {
					node = None;
					std::mem::swap(&mut node, &mut self.left);
					left_operands.push(node);
				} else {
					let (mut lvec, mut rvec) = l.get_operands(target_type);
					left_operands.append(&mut lvec);
					left_operands.append(&mut rvec);
					self.left = None;
				}
				if r.boolean_type != target_type {
					node = None;
					std::mem::swap(&mut node, &mut self.right);
					right_operands.push(node);
				} else {
					let (mut lvec, mut rvec) = r.get_operands(target_type);
					right_operands.append(&mut lvec);
					right_operands.append(&mut rvec);
					self.right = None;
				}
			},
			_ => {
				panic!("missing operand for '{}' operation", self.boolean_type);
			},
		}
		(left_operands, right_operands)
	}

	fn build_right_handed_tree_from_operand_list(
		&mut self,
		mut new_nodes: Vec<Box<Self>>,
		target_type: BooleanAstType,
	) {
		let mut tmp: Box<Self>;
		let mut new_right: Option<Box<Self>> = None;

		loop {
			match new_nodes.len() {
				0 => panic!("missing nodes"),
				1 => {
					std::mem::swap(&mut self.right, &mut new_right);
					self.left = new_nodes.pop();
					break;
				},
				_ => {
					match new_right {
						None => {
							new_right = new_nodes.pop();
						},
						Some(_) => {
							tmp = Box::new(
								Self::new(Self::type_to_symbol(target_type))
							);
							tmp.left = new_nodes.pop();
							tmp.right = new_right;
							new_right = Some(tmp);
						},
					}
				},
			}
		}
	}

	pub fn distribute(&mut self, boolean_type: BooleanAstType) {
		let inverse_type: BooleanAstType;
		let mut new_node: Box<Self>;

		match boolean_type {
			BooleanAstType::Conjunction => {
				inverse_type = BooleanAstType::Disjunction;
			},
			BooleanAstType::Disjunction => {
				inverse_type = BooleanAstType::Conjunction;
			},
			_ => panic!("cannot distribute '{}' op", boolean_type),
		}
		if self.boolean_type != boolean_type { return };
		println!("\nBEFORE get_operands(): '{}'\n", self.to_formula());
		let (left_ops, right_ops) = self.get_operands(inverse_type);
		if left_ops.len() == 0 && right_ops.len() == 0 { return };
		println!("LEFT_OPS:");
		for left_op in &left_ops {
			if let Some(l) = left_op {
				println!("{}", l.to_formula());
			} else {
				println!("None");
			}
		}
		println!("RIGHT_OPS:");
		for right_op in &right_ops {
			if let Some(r) = right_op {
				println!("{}", r.to_formula());
			} else {
				println!("None");
			}
		}
		println!("\nSTART: '{}'\n{}\n", self.to_formula(), self);
		self.change_type(inverse_type);
		let mut new_nodes: Vec<Box<Self>> = vec![];
		for left_op in &left_ops {
			for right_op in &right_ops {
				if let (Some(left), Some(right)) = (left_op, right_op) {
					println!("left: '{}'\n{}", left.to_formula(), left);
					println!("right: '{}'\n{}", right.to_formula(), right);
					new_node = Box::new(
						Self::new(Self::type_to_symbol(boolean_type))
					);
					new_node.left = Some(left.clone());
					new_node.right = Some(right.clone());
					new_nodes.push(new_node);
				} else {
					panic!("empty option operand");
				}
			}
		}
		self.build_right_handed_tree_from_operand_list(new_nodes, inverse_type);
		println!("END distribute(): '{}'\n\n", self.to_formula());
	}

	pub fn factor(&mut self, boolean_type: BooleanAstType) {
		let mut common_factor: Option<Box<Self>> = None;
		let mut term_a: Option<Box<Self>> = None;
		let mut term_b: Option<Box<Self>> = None;
		let inverse_type: BooleanAstType;
		match boolean_type {
			BooleanAstType::Conjunction => {
				inverse_type = BooleanAstType::Disjunction;
			},
			BooleanAstType::Disjunction => {
				inverse_type = BooleanAstType::Conjunction;
			},
			_ => panic!("cannot factor {} op", boolean_type),
		}
		if self.boolean_type != inverse_type { return };
		match (&mut self.left, &mut self.right) {
			(Some(left), Some(right)) => {
				if right.boolean_type != boolean_type
					|| left.boolean_type != boolean_type {
					return
				}
				match (&mut left.left, &mut left.right,
					&mut right.left, &mut right.right) {
					(Some(ll), Some(lr), Some(rl), Some(rr)) => {
						if ll == rl {
							std::mem::swap(&mut common_factor, &mut left.left);
							std::mem::swap(&mut term_a, &mut left.right);
							std::mem::swap(&mut term_b, &mut right.right);
						} else if ll == rr {
							std::mem::swap(&mut common_factor, &mut left.left);
							std::mem::swap(&mut term_a, &mut left.right);
							std::mem::swap(&mut term_b, &mut right.left);
						} else if lr == rl {
							std::mem::swap(&mut common_factor, &mut left.right);
							std::mem::swap(&mut term_a, &mut left.left);
							std::mem::swap(&mut term_b, &mut right.right);
						} else if lr == rr {
							std::mem::swap(&mut common_factor, &mut left.right);
							std::mem::swap(&mut term_a, &mut left.left);
							std::mem::swap(&mut term_b, &mut right.left);
						} else {
							return
						}
					},
					_ => {
						panic!("missing operand for '{}' operation",
							boolean_type);
					},
				}
			},
			_ => {
				panic!("missing operand for '{}' operation", self.boolean_type);
			},
		}
		self.change_type(boolean_type);
		std::mem::swap(&mut self.left, &mut common_factor);
		let mut new_right =
			Box::new(Self::new(Self::type_to_symbol(inverse_type)));
		std::mem::swap(&mut new_right.left, &mut term_a);
		std::mem::swap(&mut new_right.right, &mut term_b);
		self.right = Some(new_right);
	}

	pub fn replace_exclusive_disjunction(&mut self) {
		if self.boolean_type != BooleanAstType::ExclusiveDisjunction { return };
		match (&self.left, &self.right) {
			(Some(_), Some(_)) => {
				let mut new_right = Box::new(Self::new('!'));
				let mut copy = Box::new(self.clone());
				copy.change_type(BooleanAstType::Conjunction);
				new_right.left = Some(copy);
				let mut new_left = Box::new(Self::new('|'));
				std::mem::swap(&mut new_left.left, &mut self.left);
				std::mem::swap(&mut new_left.right, &mut self.right);
				self.change_type(BooleanAstType::Conjunction);
				self.left = Some(new_left);
				self.right = Some(new_right);
			},
			_ => {
				panic!("missing operand for '{}' operation", self.boolean_type);
			},
		}
	}

	pub fn replace_material_condition(&mut self) {
		if self.boolean_type != BooleanAstType::MaterialCondition { return };
		match (&self.left, &self.right) {
			(Some(_), Some(_)) => {
				self.change_type(BooleanAstType::Disjunction);
				let mut new_left = Box::new(Self::new('!'));
				std::mem::swap(&mut new_left.left, &mut self.left);
				self.left = Some(new_left);
			},
			_ => {
				panic!("missing operand for '{}' operation", self.boolean_type);
			},
		}
	}

	pub fn replace_logical_equivalence(&mut self) {
		if self.boolean_type != BooleanAstType::LogicalEquivalence { return };
		match (&self.left, &self.right) {
			(Some(_), Some(_)) => {
				let mut new_right = Box::new(self.clone());
				new_right.change_type(BooleanAstType::MaterialCondition);
				std::mem::swap(&mut new_right.left, &mut new_right.right);
				let mut new_left = Box::new(Self::new('>'));
				std::mem::swap(&mut new_left.left, &mut self.left);
				std::mem::swap(&mut new_left.right, &mut self.right);
				self.change_type(BooleanAstType::Conjunction);
				self.left = Some(new_left);
				self.right = Some(new_right);
			},
			_ => {
				panic!("missing operand for '{}' operation", self.boolean_type);
			},
		}
	}

	pub fn eliminate_double_negation(&mut self) {
		let mut next_useful_node: Option<Box<Self>> = None;
		if self.boolean_type != BooleanAstType::Negation { return };
		if let Some(child) = &mut self.left {
			if child.boolean_type != BooleanAstType::Negation { return };
			if let Some(_) = &mut child.left {
				std::mem::swap(&mut next_useful_node, &mut child.left);
			}
		}
		if let Some(mut grand_child) = next_useful_node {
			self.boolean_type = grand_child.boolean_type;
			self.op_symbol = grand_child.op_symbol;
			std::mem::swap(&mut self.left, &mut grand_child.left);
			std::mem::swap(&mut self.right, &mut grand_child.right);
			self.eliminate_double_negation();
		}
	}

	pub fn replace_junction_negation(&mut self) {
		let mut next_useful_node: Option<Box<Self>> = None;
		if self.boolean_type != BooleanAstType::Negation { return };
		if let Some(child) = &mut self.left {
			if child.boolean_type != BooleanAstType::Disjunction
				&& child.boolean_type != BooleanAstType::Conjunction {
				return
			};
			std::mem::swap(&mut next_useful_node, &mut self.left);
		}
		if let Some(mut child) = next_useful_node {
			let new_type = if child.boolean_type == BooleanAstType::Conjunction {
				BooleanAstType::Disjunction
			} else {
				BooleanAstType::Conjunction
			};
			self.change_type(new_type);
			let mut new_left = Box::new(Self::new('!'));
			let mut new_right = Box::new(Self::new('!'));
			std::mem::swap(&mut new_left.left, &mut child.left);
			std::mem::swap(&mut new_right.left, &mut child.right);
			new_left.eliminate_double_negation();
			new_right.eliminate_double_negation();
			self.left = Some(new_left);
			self.right = Some(new_right);
		}
	}

	pub fn replace_disjunction(&mut self) {
		self.distribute(BooleanAstType::Disjunction);
		self.factor(BooleanAstType::Conjunction);
	}

	pub fn replace_conjunction(&mut self) {
		self.distribute(BooleanAstType::Conjunction);
		self.factor(BooleanAstType::Disjunction);
	}

	pub fn left_rotate_disjunction(&mut self) {
		self.left_rotate(BooleanAstType::Disjunction);
	}

	pub fn right_rotate_disjunction(&mut self) {
		self.right_rotate(BooleanAstType::Disjunction);
	}

	pub fn right_rotate_conjunction(&mut self) {
		self.right_rotate(BooleanAstType::Conjunction);
	}

	pub fn negation_normal_form(&self) -> bool {
		match self.boolean_type {
			BooleanAstType::Variable => true,
			BooleanAstType::Negation => {
				if let (Some(child), None) = (&self.left, &self.right) {
					child.boolean_type == BooleanAstType::Variable
				} else {
					panic!("invalid '{}' op", self.boolean_type);
				}
			},
			BooleanAstType::Disjunction | BooleanAstType::Conjunction => {
				if let (Some(left), Some(right)) = (&self.left, &self.right) {
					left.negation_normal_form() && right.negation_normal_form()
				} else {
					panic!("invalid '{}' op", self.boolean_type);
				}
			},
			_ => false,
		}
	}

	fn cnf_check(&self, accept_conjunctions: bool) -> bool {
		match (self.boolean_type, accept_conjunctions) {
			(BooleanAstType::Conjunction, true) => {
				if let (Some(left), Some(right)) = (&self.left, &self.right) {
					left.cnf_check(false) && right.cnf_check(true)
				} else {
					panic!("invalid '{}' op", self.boolean_type);
				}
			},
			(BooleanAstType::Conjunction, false) => false,
			(BooleanAstType::Disjunction, _) => {
				if let (Some(left), Some(right)) = (&self.left, &self.right) {
					left.cnf_check(false) && right.cnf_check(false)
				} else {
					panic!("invalid '{}' op", self.boolean_type);
				}
			},
			(BooleanAstType::Variable, _) => true,
			(BooleanAstType::Negation, _) => true,
			_ => panic!("invalid op '{}' in CNF", self.boolean_type),
		}
	}

	pub fn conjunctive_normal_form(&self) -> bool {
		self.negation_normal_form() && self.cnf_check(true)
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
		node_opt: &Option<Box<Self>>, mut padding: String,
		pointer: &str, has_left_sibling: bool) {
		if let Some(node) = node_opt {
			tree.push_str(&format!("\n{}{}{}",
				padding, pointer, node.boolean_type));
			if node.boolean_type == BooleanAstType::Variable {
				tree.push_str(&format!("({})", node.op_symbol));
			}
			padding.push_str(if has_left_sibling { "│  " } else { "   " });
			let pointer_left = "└──";
			let pointer_right = if node.has_left() { "├──" } else { "└──" };
			Self::node_string(&mut tree, &node.right, padding.clone(),
				pointer_right, node.has_left());
			Self::node_string(&mut tree, &node.left, padding,
				pointer_left, false);
		}
	}

	fn tree_string(&self) -> String {
		let mut tree = String::new();
		let pointer_left = "└──";
		let pointer_right = if self.has_left() { "├──" } else { "└──" };

		tree.push_str(&self.boolean_type.to_string());
		if self.boolean_type == BooleanAstType::Variable {
			tree.push_str(&format!("({})", self.op_symbol));
		}
		Self::node_string(&mut tree, &self.right, String::new(),
			pointer_right, self.has_left());
		Self::node_string(&mut tree, &self.left, String::new(),
			pointer_left, false);
		tree
	}

	pub fn to_formula(&self) -> String {
		let mut formula = String::new();
		fn add_node(node: &BooleanAstNode, formula: &mut String) {
			if let Some(left) = &node.left { add_node(left, formula); }
			if let Some(right) = &node.right { add_node(right, formula); }
			formula.push(node.op_symbol);
		}
		add_node(self, &mut formula);
		formula
	}

	pub fn to_nnf(&mut self) {
		self.pre_order(Self::replace_logical_equivalence);
		self.pre_order(Self::replace_material_condition);
		self.pre_order(Self::replace_exclusive_disjunction);
		self.pre_order(Self::eliminate_double_negation);
		self.pre_order(Self::replace_junction_negation);
	}

	fn cnf(&mut self) {
		match self.boolean_type {
			BooleanAstType::Variable | BooleanAstType::Negation => (),
			BooleanAstType::Conjunction | BooleanAstType::Disjunction => {
				if let (Some(l), Some(r)) = (&mut self.left, &mut self.right) {
					l.cnf();
					r.cnf();
					if self.boolean_type == BooleanAstType::Disjunction {
						self.distribute(BooleanAstType::Disjunction);
					} else if l.boolean_type == BooleanAstType::Conjunction {
						//TODO: probably should 'get_operands' here to fix this
						self.right_rotate(BooleanAstType::Conjunction);
					}
				} else {
					panic!("missing operand for '{}' operation",
						self.boolean_type);
				}
			},
			_ => panic!("unexpected op '{}'", self.boolean_type),
		}
	}

	pub fn to_cnf(&mut self) {
		self.to_nnf();
		self.cnf();
		/*
		self.side_order(Self::replace_disjunction, Self::pre_order, true);
		*/
	}
}

pub fn negation_normal_form(formula: &str) -> String {
	let mut ast = BooleanAstNode::tree(formula);
	ast.to_nnf();
	ast.to_formula()
}

pub fn conjunctive_normal_form(formula: &str) -> String {
	let mut ast = BooleanAstNode::tree(formula);
	ast.to_cnf();
	ast.to_formula()
}
