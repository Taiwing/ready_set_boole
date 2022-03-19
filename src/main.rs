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
    print_truth_table("ABCDEFGH||=&&||!");
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
    #[should_panic(expected = "formula string is empty")]
    fn truth_table_empty_string() {
        print_truth_table("");
    }

    #[test]
    #[should_panic(expected = "'1' is not a valid op")]
    fn truth_table_boolean_value() {
        print_truth_table("100&Z!");
    }
}
