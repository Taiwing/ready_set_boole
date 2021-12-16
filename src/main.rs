mod rsb_lib;
use crate::rsb_lib::*;
use gray_codes::GrayCode32;

fn main() {
    adder_diff(1234, 4321);
    multiplier_diff(1234, 4321);
    gray_code_diff(15);
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
    println!("gray_code({}): mine = {}, orig = {}", n, mine, orig);
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
        gray_code_diff(2);
        gray_code_diff(3);
        gray_code_diff(4);
        gray_code_diff(5);
        gray_code_diff(6);
        gray_code_diff(7);
        gray_code_diff(8);
    }

    #[test]
    fn gray_code_big_numbers() {
        gray_code_diff(15);
        gray_code_diff(2414);
        gray_code_diff(1234124);
        gray_code_diff(u32::MAX/4);
        gray_code_diff(u32::MAX/3);
        gray_code_diff(u32::MAX/2);
        gray_code_diff(u32::MAX);
    }
}
