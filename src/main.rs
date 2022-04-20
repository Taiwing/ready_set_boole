mod rsb_lib;
use crate::rsb_lib::*;
use gray_codes::GrayCode32;

fn main() {
    adder_diff(1234, 4321);
    multiplier_diff(1234, 4321);
    gray_code_diff(1234);
    eval_formula("10&");
    print_truth_table("AB&");
    println!("");
    print_truth_table("AB&C|");
    println!("");
    print_truth_table("A");
    println!("");
    print_truth_table("Z!");
    println!("");
    print_truth_table("CZ&Z|!");
    println!("");

	let mut formula = "AB&C|";
	let mut ast = BooleanAstNode::tree(formula);
    println!("original:\t'{}'\nmine:\t\t'{}'\n{}\n",
		formula, ast.to_formula(), ast);

	formula = "ABC&|";
	ast = BooleanAstNode::tree(formula);
    println!("original:\t'{}'\nmine:\t\t'{}'\n{}\n",
		formula, ast.to_formula(), ast);

	formula = "CZ&Z|!";
	ast = BooleanAstNode::tree(formula);
    println!("original:\t'{}'\nmine:\t\t'{}'\n{}\n",
		formula, ast.to_formula(), ast);

	formula = "ABCDEFGH||=&&||!";
	ast = BooleanAstNode::tree(formula);
    println!("original:\t'{}'\nmine:\t\t'{}'\n{}\n",
		formula, ast.to_formula(), ast);

	formula = "ABCDEFGHI>^|=&&||!";
	ast = BooleanAstNode::tree(formula);
    println!("original:\t'{}'\nmine:\t\t'{}'\n{}\n",
		formula, ast.to_formula(), ast);
	ast.pre_order(BooleanAstNode::replace_exclusive_disjunction);
    println!("after XOR replacement:\t'{}'\n{}\n", ast.to_formula(), ast);

	formula = "PQ^A|";
	let expected = "PQ|PQ&!&A|";
	truth_diff(formula, expected);
}

fn adder_diff(left: u32, right: u32) {
    let mine: u32 = adder(left, right);
    let orig: u32 = left.wrapping_add(right);
    println!("{}+{}: mine = {}, orig = {}", left, right, mine, orig);
    assert_eq!(mine, orig);
}

fn multiplier_diff(left: u32, right: u32) {
    let mine: u32 = multiplier(left, right);
    let orig: u32 = left.wrapping_mul(right);
    println!("{}*{}: mine = {}, orig = {}", left, right, mine, orig);
    assert_eq!(mine, orig);
}

fn gray_code_diff(n: u32) {
    let mine: u32 = gray_code(n);
    let orig: u32 = GrayCode32::from_index(n);
    println!("gray_code({} [{:#b}])): mine = {} [{:#b}], orig = {} [{:#b}]",
        n, n, mine, mine, orig, orig);
    assert_eq!(mine, orig);
}

fn truth_diff(left: &str, right: &str) {
	match (truth_table::<std::io::Stdout>(left, None),
		truth_table::<std::io::Stdout>(right, None)) {
		(Some(left_truth), Some(right_truth)) => {
			println!("left:\t\t'{}'\n{}", left, left_truth);
			println!("right:\t'{}'\n{}", right, right_truth);
			assert_eq!(left_truth, right_truth);
		},
		_ => panic!("missing truth"),
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder_basics() {
        adder_diff(0, 0);
        adder_diff(1, 0);
        adder_diff(0, 1);
        adder_diff(1, 1);
        adder_diff(12, 42);
        adder_diff(1, 1234);
        adder_diff(24124, 2887979);
        adder_diff(u32::MAX, 0);
        adder_diff(u32::MAX - 1, 1);
        adder_diff(u32::MAX - 123456, 123456);
    }

    #[test]
    fn adder_overflow() {
        adder_diff(1, u32::MAX);
        adder_diff(u32::MAX, 2);
        adder_diff(2345, u32::MAX);
        adder_diff(u32::MAX, u32::MAX);
        adder_diff(u32::MAX/2, u32::MAX);
        adder_diff(u32::MAX/2, u32::MAX/3);
    }

    #[test]
    fn multiplier_basics() {
        multiplier_diff(0, 0);
        multiplier_diff(3, 6);
        multiplier_diff(9, 71);
        multiplier_diff(187, 18);
        multiplier_diff(3, 241341);
        multiplier_diff(241341, 10);
        multiplier_diff(0, u32::MAX);
        multiplier_diff(u32::MAX, 0);
        multiplier_diff(u32::MAX, 1);
        multiplier_diff(1, u32::MAX);
    }

    #[test]
    fn multiplier_overflow() {
        multiplier_diff(10, u32::MAX);
        multiplier_diff(1000, u32::MAX);
        multiplier_diff(u32::MAX, u32::MAX);
        multiplier_diff(u32::MAX/2, u32::MAX/2);
    }

    #[test]
    fn gray_code_basics() {
        gray_code_diff(0);
        gray_code_diff(1);
    }

    #[test]
    fn gray_code_less_basic() {
        for i in 0..16 {
            gray_code_diff(i);
        }
    }

    #[test]
    fn gray_code_big_numbers() {
        gray_code_diff(92948);
        gray_code_diff(2414);
        gray_code_diff(1234124);
        gray_code_diff(u32::MAX/4);
        gray_code_diff(u32::MAX/3);
        gray_code_diff(u32::MAX/2);
        gray_code_diff(u32::MAX/2 + 1);
        gray_code_diff(u32::MAX - 1);
        gray_code_diff(u32::MAX);
    }

    #[test]
    fn bool_eval_subject_tests() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }

    #[test]
    fn bool_eval_basic_tests() {
        assert_eq!(eval_formula("0"), false);
        assert_eq!(eval_formula("1"), true);
        assert_eq!(eval_formula("0!"), true);
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("00&"), false);
        assert_eq!(eval_formula("01&"), false);
        assert_eq!(eval_formula("11&"), true);
        assert_eq!(eval_formula("00|"), false);
        assert_eq!(eval_formula("01|"), true);
        assert_eq!(eval_formula("11|"), true);
        assert_eq!(eval_formula("00^"), false);
        assert_eq!(eval_formula("01^"), true);
        assert_eq!(eval_formula("10^"), true);
        assert_eq!(eval_formula("11^"), false);
        assert_eq!(eval_formula("00>"), true);
        assert_eq!(eval_formula("01>"), true);
        assert_eq!(eval_formula("10>"), false);
        assert_eq!(eval_formula("00="), true);
        assert_eq!(eval_formula("01="), false);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("11="), true);
    }

    #[test]
    fn bool_eval_negate() {
        assert_eq!(eval_formula("10&!"), !false);
        assert_eq!(eval_formula("10|!"), !true);
        assert_eq!(eval_formula("11>!"), !true);
        assert_eq!(eval_formula("10=!"), !false);
        assert_eq!(eval_formula("1011||=!"), !true);
        assert_eq!(eval_formula("0!!"), !true);
        assert_eq!(eval_formula("1!!"), !false);
    }

    #[test]
    #[should_panic(expected = "formula string is empty")]
    fn bool_eval_empty_string() {
        eval_formula("");
    }

    #[test]
    #[should_panic(expected = "'x' is not a valid op or is missing an argument")]
    fn bool_eval_invalid_characters() {
        eval_formula("10&!xslkfj");
    }

    #[test]
    #[should_panic(expected = "'!' is not a valid op or is missing an argument")]
    fn bool_eval_no_argument_for_negate_op() {
        eval_formula("!");
    }

    #[test]
    #[should_panic(expected = "'&' is not a valid op or is missing an argument")]
    fn bool_eval_only_one_argument_for_and_op() {
        eval_formula("1&");
    }

    #[test]
    #[should_panic(expected = "the stack should be empty at the end")]
    fn bool_eval_unused_leading_zeroes() {
        assert_eq!(eval_formula("000001"), true);
    }

    #[test]
    #[should_panic(expected = "the stack should be empty at the end")]
    fn bool_eval_unused_leading_ones() {
        assert_eq!(eval_formula("111110"), false);
    }

	#[test]
	fn truth_table_subject_tests() {
		let expected: &str = "\
			| A | B | C | = |\n\
			|---|---|---|---|\n\
			| 0 | 0 | 0 | 0 |\n\
			| 0 | 0 | 1 | 1 |\n\
			| 0 | 1 | 0 | 0 |\n\
			| 0 | 1 | 1 | 1 |\n\
			| 1 | 0 | 0 | 0 |\n\
			| 1 | 0 | 1 | 1 |\n\
			| 1 | 1 | 0 | 1 |\n\
			| 1 | 1 | 1 | 1 |\n\
		";
		if let Some(truth) = truth_table::<std::io::Stdout>("AB&C|", None) {
			assert_eq!(truth, expected);
		} else {
			panic!("no output string for truth_table function");
		}
	}

    #[test]
    #[should_panic(expected = "formula string is empty")]
    fn truth_table_empty_string() {
        truth_table::<std::io::Stdout>("", None);
    }

    #[test]
    #[should_panic(expected = "'1' is not a valid op")]
    fn truth_table_boolean_value() {
        truth_table::<std::io::Stdout>("100&Z!", None);
    }

	#[test]
	fn bool_ast_tree_basics() {
		let mut formula = "A";
		let mut ast = BooleanAstNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "AB|";
		ast = BooleanAstNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "AB&C|";
		ast = BooleanAstNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "ABC&|";
		ast = BooleanAstNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "AB=";
		ast = BooleanAstNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "AB=!";
		ast = BooleanAstNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "ABCDEFGH||=&&||!";
		ast = BooleanAstNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "ABCDEFGHI>^|=&&||!";
		ast = BooleanAstNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());
	}

	#[test]
    #[should_panic(expected = "formula string is empty")]
	fn bool_ast_tree_empty_string() {
		BooleanAstNode::tree("");
	}

	#[test]
    #[should_panic(expected = "unused operands in formula string")]
	fn bool_ast_tree_unused_operands() {
		BooleanAstNode::tree("ABC|");
	}

	#[test]
    #[should_panic(expected = "missing operand for 'Negation' operation")]
	fn bool_ast_tree_missing_operand() {
		BooleanAstNode::tree("!");
	}

	#[test]
	fn bool_ast_replace_exclusive_disjunction() {
		let mut formula = "PQ^";
		let mut expected = "PQ|PQ&!&";
		let mut ast = BooleanAstNode::tree(formula);
		ast.pre_order(BooleanAstNode::replace_exclusive_disjunction);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "PQ^A|";
		expected = "PQ|PQ&!&A|";
		ast = BooleanAstNode::tree(formula);
		ast.pre_order(BooleanAstNode::replace_exclusive_disjunction);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "ABCDEFGHI>^|=^^||!";
		ast = BooleanAstNode::tree(formula);
		ast.pre_order(BooleanAstNode::replace_exclusive_disjunction);
		truth_diff(formula, &ast.to_formula());

		formula = "ABCDEFGHI>^^=^^^^!";
		ast = BooleanAstNode::tree(formula);
		ast.pre_order(BooleanAstNode::replace_exclusive_disjunction);
		truth_diff(formula, &ast.to_formula());
	}

	#[test]
	fn bool_ast_replace_material_condition() {
		let mut formula = "AB>";
		let mut expected = "A!B|";
		let mut ast = BooleanAstNode::tree(formula);
		ast.pre_order(BooleanAstNode::replace_material_condition);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "ABC|>";
		expected = "A!BC||";
		ast = BooleanAstNode::tree(formula);
		ast.pre_order(BooleanAstNode::replace_material_condition);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "AB^CD|>";
		expected = "AB^!CD||";
		ast = BooleanAstNode::tree(formula);
		ast.pre_order(BooleanAstNode::replace_material_condition);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "ABCDEFGHI>>^=>>||!";
		ast = BooleanAstNode::tree(formula);
		ast.pre_order(BooleanAstNode::replace_material_condition);
		truth_diff(formula, &ast.to_formula());
	}

	/*
	#[test]
	fn nnf_subject_tests() {
		assert_eq!("A!B!|", negation_normal_form("AB&!"));
		assert_eq!("A!B!&", negation_normal_form("AB|!"));
		assert_eq!("A!B|", negation_normal_form("AB>"));
		assert_eq!("AB&A!B!&|", negation_normal_form("AB="));
		assert_eq!("A!B!&C!|", negation_normal_form("AB|C&!"));
	}
	*/
}
