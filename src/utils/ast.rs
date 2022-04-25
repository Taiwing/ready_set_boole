use std::fmt;
use super::truth;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoolType {
	Variable,
	Negation,
	Conjunction,
	Disjunction,
	ExclusiveDisjunction,
	MaterialCondition,
	LogicalEquivalence,
}

impl fmt::Display for BoolType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug)]
pub struct BoolNode {
	boolean_type: BoolType,
	op_symbol: char,
	left: Option<Box<Self>>,
	right: Option<Box<Self>>,
}

impl fmt::Display for BoolNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.tree_string())
	}
}

impl Clone for BoolNode {
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

impl PartialEq for BoolNode {
	/// This is not the smartest way to do this, but it was already done lol
	fn eq(&self, other: &Self) -> bool {
		let left_formula = self.to_formula();
		let right_formula = other.to_formula();
		if left_formula == right_formula { return true };
		match (truth::table::<std::io::Stdout>(&left_formula, None),
			truth::table::<std::io::Stdout>(&right_formula, None)) {
			(Some(left_truth), Some(right_truth)) => {
				left_truth == right_truth
			},
			_ => panic!("missing truth"),
		}
	}
}

impl Eq for BoolNode {}

impl BoolNode {
	fn symbol_to_type(c: char) -> BoolType {
		match c {
			'A' ..='Z' => BoolType::Variable,
			'!' => BoolType::Negation,
			'&' => BoolType::Conjunction,
			'|' => BoolType::Disjunction,
			'^' => BoolType::ExclusiveDisjunction,
			'>' => BoolType::MaterialCondition,
			'=' => BoolType::LogicalEquivalence,
			_ => panic!("'{}' is not a valid op", c),
		}
	}

	fn type_to_symbol(boolean_type: BoolType) -> char {
		match boolean_type {
			BoolType::Negation => '!',
			BoolType::Conjunction => '&',
			BoolType::Disjunction => '|',
			BoolType::ExclusiveDisjunction => '^',
			BoolType::MaterialCondition => '>',
			BoolType::LogicalEquivalence => '=',
			BoolType::Variable => {
				panic!("no predefined symbol for '{}' type", boolean_type);
			},
		}
	}

	fn change_type(&mut self, new_type: BoolType) {
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
			BoolType::Variable => (),
			BoolType::Negation => {
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

	pub fn is_valid_rotation(&self, boolean_type: BoolType, is_left: bool) -> bool {
		match boolean_type {
			BoolType::Variable => {
				panic!("cannot rotate '{}' node because it has no operand",
					boolean_type);
			},
			BoolType::Negation => {
				panic!("cannot rotate '{}' node because it only has 1 operand",
					boolean_type);
			},
			BoolType::MaterialCondition => {
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

	pub fn right_rotate(&mut self, boolean_type: BoolType) {
		if self.is_valid_rotation(boolean_type, false) == false { return };
		std::mem::swap(&mut self.left, &mut self.right);
		if let Some(right) = &mut self.right {
			std::mem::swap(&mut right.left, &mut right.right);
			std::mem::swap(&mut self.left, &mut right.right);
			std::mem::swap(&mut self.op_symbol, &mut right.op_symbol);
			std::mem::swap(&mut self.boolean_type, &mut right.boolean_type);
		}
	}

	pub fn left_rotate(&mut self, boolean_type: BoolType) {
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
	) -> (BoolType, char, Option<Box<Self>>, Option<Box<Self>>) {
		(self.boolean_type, self.op_symbol, self.left, self.right)
	}

	fn get_lr_operands(
		&mut self,
		target_type: BoolType,
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
					let (mut lvec, mut rvec) = l.get_lr_operands(target_type);
					left_operands.append(&mut lvec);
					left_operands.append(&mut rvec);
					self.left = None;
				}
				if r.boolean_type != target_type {
					node = None;
					std::mem::swap(&mut node, &mut self.right);
					right_operands.push(node);
				} else {
					let (mut lvec, mut rvec) = r.get_lr_operands(target_type);
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

	fn get_operands(
		&mut self,
		target_type: BoolType
	) -> Vec<Option<Box<Self>>> {
		let mut operands: Vec<Option<Box<Self>>> = vec![];

		if self.boolean_type == target_type {
			let (mut lvec, mut rvec) = self.get_lr_operands(target_type);
			operands.append(&mut lvec);
			operands.append(&mut rvec);
		}
		operands
	}

	fn build_right_handed_tree_from_operand_list(
		&mut self,
		mut new_nodes: Vec<Box<Self>>,
		target_type: BoolType,
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

	pub fn distribute(&mut self, boolean_type: BoolType) {
		let inverse_type: BoolType;
		let mut new_node: Box<Self>;

		match boolean_type {
			BoolType::Conjunction => {
				inverse_type = BoolType::Disjunction;
			},
			BoolType::Disjunction => {
				inverse_type = BoolType::Conjunction;
			},
			_ => panic!("cannot distribute '{}' op", boolean_type),
		}
		if self.boolean_type != boolean_type { return };
		let (left_ops, right_ops) = self.get_lr_operands(inverse_type);
		if left_ops.len() == 0 && right_ops.len() == 0 { return };
		self.change_type(inverse_type);
		let mut new_nodes: Vec<Box<Self>> = vec![];
		for left_op in &left_ops {
			for right_op in &right_ops {
				if let (Some(left), Some(right)) = (left_op, right_op) {
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
	}

	pub fn factor(&mut self, boolean_type: BoolType) {
		let mut common_factor: Option<Box<Self>> = None;
		let mut term_a: Option<Box<Self>> = None;
		let mut term_b: Option<Box<Self>> = None;
		let inverse_type: BoolType;
		match boolean_type {
			BoolType::Conjunction => {
				inverse_type = BoolType::Disjunction;
			},
			BoolType::Disjunction => {
				inverse_type = BoolType::Conjunction;
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
		if self.boolean_type != BoolType::ExclusiveDisjunction { return };
		match (&self.left, &self.right) {
			(Some(_), Some(_)) => {
				let mut new_right = Box::new(Self::new('!'));
				let mut copy = Box::new(self.clone());
				copy.change_type(BoolType::Conjunction);
				new_right.left = Some(copy);
				let mut new_left = Box::new(Self::new('|'));
				std::mem::swap(&mut new_left.left, &mut self.left);
				std::mem::swap(&mut new_left.right, &mut self.right);
				self.change_type(BoolType::Conjunction);
				self.left = Some(new_left);
				self.right = Some(new_right);
			},
			_ => {
				panic!("missing operand for '{}' operation", self.boolean_type);
			},
		}
	}

	pub fn replace_material_condition(&mut self) {
		if self.boolean_type != BoolType::MaterialCondition { return };
		match (&self.left, &self.right) {
			(Some(_), Some(_)) => {
				self.change_type(BoolType::Disjunction);
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
		if self.boolean_type != BoolType::LogicalEquivalence { return };
		match (&self.left, &self.right) {
			(Some(_), Some(_)) => {
				let mut new_right = Box::new(self.clone());
				new_right.change_type(BoolType::MaterialCondition);
				std::mem::swap(&mut new_right.left, &mut new_right.right);
				let mut new_left = Box::new(Self::new('>'));
				std::mem::swap(&mut new_left.left, &mut self.left);
				std::mem::swap(&mut new_left.right, &mut self.right);
				self.change_type(BoolType::Conjunction);
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
		if self.boolean_type != BoolType::Negation { return };
		if let Some(child) = &mut self.left {
			if child.boolean_type != BoolType::Negation { return };
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
		if self.boolean_type != BoolType::Negation { return };
		if let Some(child) = &mut self.left {
			if child.boolean_type != BoolType::Disjunction
				&& child.boolean_type != BoolType::Conjunction {
				return
			};
			std::mem::swap(&mut next_useful_node, &mut self.left);
		}
		if let Some(mut child) = next_useful_node {
			let new_type = if child.boolean_type == BoolType::Conjunction {
				BoolType::Disjunction
			} else {
				BoolType::Conjunction
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
		self.distribute(BoolType::Disjunction);
		self.factor(BoolType::Conjunction);
	}

	pub fn replace_conjunction(&mut self) {
		self.distribute(BoolType::Conjunction);
		self.factor(BoolType::Disjunction);
	}

	pub fn left_rotate_disjunction(&mut self) {
		self.left_rotate(BoolType::Disjunction);
	}

	pub fn right_rotate_disjunction(&mut self) {
		self.right_rotate(BoolType::Disjunction);
	}

	pub fn right_rotate_conjunction(&mut self) {
		self.right_rotate(BoolType::Conjunction);
	}

	pub fn negation_normal_form(&self) -> bool {
		match self.boolean_type {
			BoolType::Variable => true,
			BoolType::Negation => {
				if let (Some(child), None) = (&self.left, &self.right) {
					child.boolean_type == BoolType::Variable
				} else {
					panic!("invalid '{}' op", self.boolean_type);
				}
			},
			BoolType::Disjunction | BoolType::Conjunction => {
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
			(BoolType::Conjunction, true) => {
				if let (Some(left), Some(right)) = (&self.left, &self.right) {
					left.cnf_check(false) && right.cnf_check(true)
				} else {
					panic!("invalid '{}' op", self.boolean_type);
				}
			},
			(BoolType::Conjunction, false) => false,
			(BoolType::Disjunction, _) => {
				if let (Some(left), Some(right)) = (&self.left, &self.right) {
					left.cnf_check(false) && right.cnf_check(false)
				} else {
					panic!("invalid '{}' op", self.boolean_type);
				}
			},
			(BoolType::Variable, _) => true,
			(BoolType::Negation, _) => true,
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
			if node.boolean_type == BoolType::Variable {
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
		if self.boolean_type == BoolType::Variable {
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
		fn add_node(node: &BoolNode, formula: &mut String) {
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
		let mut operands: Vec<Box<Self>> = vec![];

		match (self.boolean_type, &mut self.left, &mut self.right) {
			(BoolType::Variable | BoolType::Negation, _, _) => (),
			(
				BoolType::Conjunction | BoolType::Disjunction,
				Some(l),
				Some(r)
			) => {
				l.cnf();
				r.cnf();
				if self.boolean_type == BoolType::Disjunction {
					self.distribute(BoolType::Disjunction);
					if self.boolean_type != BoolType::Disjunction {
						return
					}
				}
				let ops = self.get_operands(self.boolean_type);
				for op in ops { operands.push(op.unwrap()); }
				self.build_right_handed_tree_from_operand_list(
					operands,
					self.boolean_type,
				);
			},
			(BoolType::Conjunction | BoolType::Disjunction, _, _) => {
				panic!("missing operand for '{}' operation", self.boolean_type);
			},
			_ => panic!("unexpected op '{}'", self.boolean_type),
		}
	}

	pub fn to_cnf(&mut self) {
		self.to_nnf();
		self.cnf();
	}
}
