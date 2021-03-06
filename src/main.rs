#![allow(dead_code)]
#![allow(unused_imports)]

mod rsb_lib;
use rsb_lib::*;
mod utils;
use utils::{ast::*, truth::{self, eval_formula, sat}, sets::*};
use gray_codes::GrayCode32;

fn main() {
	/* put your tests here :) */
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
	match (truth::table::<std::io::Stdout>(left, None),
		truth::table::<std::io::Stdout>(right, None)) {
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
		if let Some(truth) = truth::table::<std::io::Stdout>("AB&C|", None) {
			assert_eq!(truth, expected);
		} else {
			panic!("no output string for truth::table function");
		}
	}

    #[test]
    #[should_panic(expected = "formula string is empty")]
    fn truth_table_empty_string() {
        truth::table::<std::io::Stdout>("", None);
    }

    #[test]
    #[should_panic(expected = "'1' is not a valid op")]
    fn truth_table_boolean_value() {
        truth::table::<std::io::Stdout>("100&Z!", None);
    }

	#[test]
	fn bool_ast_tree_basics() {
		let mut formula = "A";
		let mut ast = BoolNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "AB|";
		ast = BoolNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "AB&C|";
		ast = BoolNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "ABC&|";
		ast = BoolNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "AB=";
		ast = BoolNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "AB=!";
		ast = BoolNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "ABCDEFGH||=&&||!";
		ast = BoolNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());

		formula = "ABCDEFGHI>^|=&&||!";
		ast = BoolNode::tree(formula);
		assert_eq!(formula, &ast.to_formula());
	}

	#[test]
    #[should_panic(expected = "formula string is empty")]
	fn bool_ast_tree_empty_string() {
		BoolNode::tree("");
	}

	#[test]
    #[should_panic(expected = "unused operands in formula string")]
	fn bool_ast_tree_unused_operands() {
		BoolNode::tree("ABC|");
	}

	#[test]
    #[should_panic(expected = "missing operand for 'Negation' operation")]
	fn bool_ast_tree_missing_operand() {
		BoolNode::tree("!");
	}

	#[test]
	fn bool_ast_replace_exclusive_disjunction() {
		let mut formula = "PQ^";
		let mut expected = "PQ|PQ&!&";
		let mut ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_exclusive_disjunction);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "PQ^A|";
		expected = "PQ|PQ&!&A|";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_exclusive_disjunction);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "ABCDEFGHI>^|=^^||!";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_exclusive_disjunction);
		truth_diff(formula, &ast.to_formula());

		formula = "ABCDEFGHI>^^=^^^^!";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_exclusive_disjunction);
		truth_diff(formula, &ast.to_formula());
	}

	#[test]
	fn bool_ast_replace_material_condition() {
		let mut formula = "AB>";
		let mut expected = "A!B|";
		let mut ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_material_condition);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "ABC|>";
		expected = "A!BC||";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_material_condition);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "AB^CD|>";
		expected = "AB^!CD||";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_material_condition);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "ABCDEFGHI>>^=>>||!";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_material_condition);
		truth_diff(formula, &ast.to_formula());
	}

	#[test]
	fn bool_ast_replace_logical_equivalence() {
		let mut formula = "AB=";
		let mut expected = "AB>BA>&";
		let mut ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_logical_equivalence);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "ABC|=";
		expected = "ABC|>BC|A>&";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_logical_equivalence);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "AB^CD|=";
		expected = "AB^CD|>CD|AB^>&";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_logical_equivalence);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "ABCDEFGHI>==^====!";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_material_condition);
		truth_diff(formula, &ast.to_formula());
	}

	#[test]
	fn bool_ast_eliminate_double_negation() {
		let mut formula = "A!!";
		let mut expected = "A";
		let mut ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::eliminate_double_negation);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "A!!!!!B|";
		expected = "A!B|";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::eliminate_double_negation);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "A!!!!!!B!!|";
		expected = "AB|";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::eliminate_double_negation);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "AB|!!!!!!!CD!E|^!!|";
		expected = "AB|!CD!E|^|";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::eliminate_double_negation);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);
	}

	#[test]
	fn bool_ast_replace_junction_negation() {
		let mut formula = "AB&!";
		let mut expected = "A!B!|";
		let mut ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_junction_negation);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "AB|!";
		expected = "A!B!&";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_junction_negation);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "AB|!!";
		expected = "A!B!&!";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_junction_negation);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);

		formula = "AB&CD^|!";
		expected = "A!B!|CD^!&";
		ast = BoolNode::tree(formula);
		ast.pre_order(BoolNode::replace_junction_negation);
		assert_eq!(ast.to_formula(), expected);
		truth_diff(formula, expected);
	}

	#[test]
	fn nnf_subject_tests() {
		let mut formula = "AB&!";
		let mut ast = BoolNode::tree(formula);
		ast.to_nnf();
		assert!(ast.negation_normal_form());
		assert_eq!("A!B!|", negation_normal_form(formula));

		formula = "AB|!";
		ast = BoolNode::tree(formula);
		ast.to_nnf();
		assert!(ast.negation_normal_form());
		assert_eq!("A!B!&", negation_normal_form(formula));

		formula = "AB>";
		ast = BoolNode::tree(formula);
		ast.to_nnf();
		assert!(ast.negation_normal_form());
		assert_eq!("A!B|", negation_normal_form(formula));

		formula = "AB=";
		ast = BoolNode::tree(formula);
		ast.to_nnf();
		assert!(ast.negation_normal_form());
		//assert_eq!("AB&A!B!&|", negation_normal_form("AB="));
		truth_diff("AB&A!B!&|", &negation_normal_form(formula));

		formula = "AB|C&!";
		ast = BoolNode::tree(formula);
		ast.to_nnf();
		assert!(ast.negation_normal_form());
		assert_eq!("A!B!&C!|", negation_normal_form(formula));
	}

	#[test]
	fn nnf_harder_tests() {
		let formula = "AB&!CD|!EF>!GH^!&!&!&!";
		let mut ast = BoolNode::tree(formula);
		ast.to_nnf();
		assert!(ast.negation_normal_form());
		truth_diff(formula, &negation_normal_form(formula));
	}

	#[test]
	fn rotations() {
		let mut formula = "AB|C|";
		let mut ast = BoolNode::tree(formula);
		ast.right_rotate(BoolType::Disjunction);
		assert_eq!("ABC||", ast.to_formula());
		ast.left_rotate(BoolType::Disjunction);
		assert_eq!(formula, ast.to_formula());

		formula = "AB&C&";
		ast = BoolNode::tree(formula);
		ast.right_rotate(BoolType::Conjunction);
		assert_eq!("ABC&&", ast.to_formula());
		ast.left_rotate(BoolType::Conjunction);
		assert_eq!(formula, ast.to_formula());

		formula = "AB&C|";
		ast = BoolNode::tree(formula);
		ast.right_rotate(BoolType::Disjunction);
		assert_eq!(formula, ast.to_formula());
		ast.left_rotate(BoolType::Disjunction);
		assert_eq!(formula, ast.to_formula());
	}

	#[test]
	fn distribute() {
		let formula = "ABC&|";
		let mut ast = BoolNode::tree(formula);
		ast.distribute(BoolType::Disjunction);
		assert_eq!("AB|AC|&", ast.to_formula());
		truth_diff(formula, &ast.to_formula());

		let formula = "AD&C|";
		ast = BoolNode::tree(formula);
		ast.distribute(BoolType::Disjunction);
		assert_eq!("AC|DC|&", ast.to_formula());
		truth_diff(formula, &ast.to_formula());

		let formula = "ABC|&";
		ast = BoolNode::tree(formula);
		ast.distribute(BoolType::Conjunction);
		assert_eq!("AB&AC&|", ast.to_formula());
		truth_diff(formula, &ast.to_formula());

		let formula = "AD|C&";
		ast = BoolNode::tree(formula);
		ast.distribute(BoolType::Conjunction);
		assert_eq!("AC&DC&|", ast.to_formula());
		truth_diff(formula, &ast.to_formula());
	}

	#[test]
	fn factor() {
		let formula = "AB|AC|&";
		let mut ast = BoolNode::tree(formula);
		ast.factor(BoolType::Disjunction);
		assert_eq!("ABC&|", ast.to_formula());
		truth_diff(formula, &ast.to_formula());

		let formula = "AB&AC&|";
		ast = BoolNode::tree(formula);
		ast.factor(BoolType::Conjunction);
		assert_eq!("ABC|&", ast.to_formula());
		truth_diff(formula, &ast.to_formula());

		let formula = "AB|CA|&";
		ast = BoolNode::tree(formula);
		ast.factor(BoolType::Disjunction);
		assert_eq!("ABC&|", ast.to_formula());
		truth_diff(formula, &ast.to_formula());

		let formula = "BA|AC|&";
		ast = BoolNode::tree(formula);
		ast.factor(BoolType::Disjunction);
		assert_eq!("ABC&|", ast.to_formula());
		truth_diff(formula, &ast.to_formula());

		let formula = "BA|CA|&";
		ast = BoolNode::tree(formula);
		ast.factor(BoolType::Disjunction);
		assert_eq!("ABC&|", ast.to_formula());
		truth_diff(formula, &ast.to_formula());
	}

	#[test]
	fn cnf_subject_tests() {
		let mut formula = "AB&!";
		let mut ast = BoolNode::tree(formula);
		ast.to_cnf();
		assert!(ast.conjunctive_normal_form());
		assert_eq!("A!B!|", conjunctive_normal_form(formula));

		formula = "AB|!";
		ast = BoolNode::tree(formula);
		ast.to_cnf();
		assert!(ast.conjunctive_normal_form());
		assert_eq!("A!B!&", conjunctive_normal_form(formula));

		formula = "AB|C&";
		ast = BoolNode::tree(formula);
		ast.to_cnf();
		assert!(ast.conjunctive_normal_form());
		assert_eq!("AB|C&", conjunctive_normal_form(formula));

		formula = "AB|C|D|";
		ast = BoolNode::tree(formula);
		ast.to_cnf();
		assert!(ast.conjunctive_normal_form());
		assert_eq!("ABCD|||", conjunctive_normal_form(formula));

		formula = "AB&C&D&";
		ast = BoolNode::tree(formula);
		ast.to_cnf();
		assert!(ast.conjunctive_normal_form());
		assert_eq!("ABCD&&&", conjunctive_normal_form(formula));

		formula = "ABCD&|&";
		ast = BoolNode::tree(formula);
		ast.to_cnf();
		assert!(ast.conjunctive_normal_form());
		assert_eq!("ABC|BD|&&", conjunctive_normal_form(formula));
	}

	#[test]
	fn cnf_harder_tests() {
		fn cnf_hard_test(formula: &str) {
			let orig = BoolNode::tree(formula);
			let mut mine = orig.clone();
			mine.to_cnf();
			assert_eq!(orig, mine);
			println!("orig: '{}'\n{}", orig.to_formula(), orig);
			println!("mine: '{}'\n{}\n", mine.to_formula(), mine);
			assert!(mine.conjunctive_normal_form());
		}

		cnf_hard_test("AB&CD||");
		cnf_hard_test("DEFGH|IJKL|&|&|&|");

		cnf_hard_test("GH|IJKL|&|&");
		cnf_hard_test("FGH|IJKL|&|&|");
		cnf_hard_test("EFGH|IJKL|&|&|&");
		cnf_hard_test("CDEFGH|IJKL|&|&|&|&");
		cnf_hard_test("ABCDEFGH|IJKL|&|&|&|&|&");

		cnf_hard_test("ABCDEFG!H|IJKL|&|&|&|&|&");
		cnf_hard_test("ABCDEFGH>IJKL|&|&|&|&|&");

		cnf_hard_test("ABCDE&||&");
		cnf_hard_test("AB&C|D|EF&&");
		cnf_hard_test("BF|DF|&G!H||BDFIJ||||BDFI|||KL||&&");
		cnf_hard_test("ABC|BD|E|BD&F|G!H||BDF||I|J|BDFI|||KL||&&&&&");
		cnf_hard_test("BF&DF&&G!H&&BDFIJ&&&&BDFI|||KL||&&");

		cnf_hard_test("BF&DF&&G!H&&BDFIJ&&&&BDFI||&KL||&&");
		cnf_hard_test("BF&DF&&G!H&&BDFIJ&&&&BDFI|&|KL||&&");
		cnf_hard_test("BF&DF&&G!H&&BDFIJ&&&&BDFI&||KL||&&");
		cnf_hard_test("BF&DF&&G!H&&BDFIJ&&&&BDFI&&|KL||&&");
		cnf_hard_test("BF&DF&&G!H&&BDFIJ&&&&BDFI&&&KL||&&");
		cnf_hard_test("BF&DF&&G!H&&BDFIJ&&&&BDFI&&&KL&|&&");
	}

	#[test]
	fn sat_subject_tests() {
		assert_eq!(sat("AB|"), true);
		assert_eq!(sat("AB&"), true);
		assert_eq!(sat("AA!&"), false);
		assert_eq!(sat("AA^"), false);
	}

	#[test]
	fn powerset_basics() {
		let input: Vec<i32> = vec![];
		let pow = powerset(&input);
		assert_eq!(pow.len(), 2usize.pow(input.len() as u32));

		let input: Vec<i32> = vec![0];
		let pow = powerset(&input);
		assert_eq!(pow.len(), 2usize.pow(input.len() as u32));

		let input: Vec<i32> = vec![0, 1];
		let pow = powerset(&input);
		assert_eq!(pow.len(), 2usize.pow(input.len() as u32));

		let input: Vec<i32> = vec![0, 1, 2];
		let pow = powerset(&input);
		assert_eq!(pow.len(), 2usize.pow(input.len() as u32));

		let input: Vec<i32> = vec![0, 1, 2, 3];
		let pow = powerset(&input);
		assert_eq!(pow.len(), 2usize.pow(input.len() as u32));

		let input: Vec<i32> = vec![0, 1, 2, 3, 4];
		let pow = powerset(&input);
		assert_eq!(pow.len(), 2usize.pow(input.len() as u32));

		let input: Vec<i32> = (0..10).collect();
		let pow = powerset(&input);
		assert_eq!(pow.len(), 2usize.pow(input.len() as u32));
	}

	#[test]
	#[should_panic(expected = "input is waaaaay too big, please calm down")]
	fn powerset_big_input() {
		let input: Vec<i32> = (0..128).collect();
		powerset(&input);
	}

	#[test]
	fn eval_set_subject_tests() {
		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 3, 4]];
		let result = eval_set("AB&", &sets);
		assert_eq!(result, vec![0]);

		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![3, 4, 5]];
		let result = eval_set("AB|", &sets);
		assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);

		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2]];
		let result = eval_set("A!", &sets);
		assert_eq!(result, vec![]);
	}

	#[test]
	fn eval_set_other_ops() {
		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 3, 4]];
		let result = eval_set("AB^", &sets);
		assert_eq!(result, vec![1, 2, 3, 4]);

		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2, 5], vec![0, 3, 4, 5, 6]];
		let result = eval_set("AB^!", &sets);
		assert_eq!(result, vec![0, 5]);

		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 3, 4]];
		let result = eval_set("AB>", &sets);
		assert_eq!(result, vec![0, 3, 4]);

		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 3, 4]];
		let result = eval_set("AB>!", &sets);
		assert_eq!(result, vec![1, 2]);

		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 1, 2], vec![55]];
		let result = eval_set("AB=CC^|", &sets);
		assert_eq!(result, vec![0, 1, 2]);

		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 1, 2], vec![55]];
		let result = eval_set("AB=CC^|!", &sets);
		assert_eq!(result, vec![55]);

		let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 1, 2]];
		let result = eval_set("AB=", &sets);
		assert_eq!(result, vec![0, 1, 2]);
	}

	#[test]
	fn map_is_in_range() {
		let (x, y) = (0, 0);
		let result = map(x, y);
		assert_eq!(result, 0.0);

		let (x, y) = (0, 1);
		let result = map(x, y);
		assert!(result != 0.0);

		for coordinates in (0..=u32::MAX).step_by(4242) {
			let x = (coordinates & 0xffff) as u16;
			let y = ((coordinates & 0xffff0000) >> 16) as u16;
			let result = map(x, y);
			assert!(result.is_sign_positive());
			assert!(result.is_nan() == false);
			assert!(result <= 1.0);
		}
	}

	#[test]
	fn map_to_reverse_map() {
		for coordinates in (0..=u32::MAX).step_by(4444) {
			let x_in = (coordinates & 0xffff) as u16;
			let y_in = ((coordinates & 0xffff0000) >> 16) as u16;
			let result = map(x_in, y_in);
			let (x_out, y_out) = reverse_map(result);
			assert_eq!(x_out, x_in);
			assert_eq!(y_out, y_in);
		}
	}

	#[test]
	fn reverse_map_to_map() {
		let n_in = 0.0;
		let (x, y) = reverse_map(n_in);
		let n_out = map(x, y);
		assert_eq!(n_out, n_in);

		let n_in = 0.25;
		let (x, y) = reverse_map(n_in);
		let n_out = map(x, y);
		assert_eq!(n_out, n_in);

		let n_in = 0.5;
		let (x, y) = reverse_map(n_in);
		let n_out = map(x, y);
		assert_eq!(n_out, n_in);

		let n_in = 0.75;
		let (x, y) = reverse_map(n_in);
		let n_out = map(x, y);
		assert_eq!(n_out, n_in);

		let n_in = 1.0;
		let (x, y) = reverse_map(n_in);
		let n_out = map(x, y);
		assert_eq!(n_out, n_in);

		let n_in = f64::EPSILON;
		let (x, y) = reverse_map(n_in);
		let n_out = map(x, y);
		assert_eq!(n_out, n_in);
	}

	#[test]
    #[should_panic(expected = "input number '2' is out of range [0;1]")]
	fn reverse_map_too_big() {
		reverse_map(2.0);
	}

	#[test]
    #[should_panic(expected = "input number 'NaN' is out of range [0;1]")]
	fn reverse_map_nan() {
		reverse_map(f64::NAN);
	}

	#[test]
    #[should_panic(expected = "input number '-42' is out of range [0;1]")]
	fn reverse_map_negative() {
		reverse_map(-42.0);
	}
}
