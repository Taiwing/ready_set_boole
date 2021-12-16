fn main() {
    test_adder(1234, 4321);
    test_multiplier(1234, 4321);
}

fn adder(mut a: u32, mut b: u32) -> u32 {
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

fn multiplier(mut a: u32, mut b: u32) -> u32 {
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

fn test_adder(left: u32, right: u32) {
    let mine: u32 = adder(left, right);
    let orig: u32 = left.wrapping_add(right);
    println!("{}+{}: mine = {}, orig = {}", left, right, mine, orig);
    assert_eq!(mine, orig);
}

fn test_multiplier(left: u32, right: u32) {
    let mine: u32 = multiplier(left, right);
    let orig: u32 = left.wrapping_mul(right);
    println!("{}*{}: mine = {}, orig = {}", left, right, mine, orig);
    assert_eq!(mine, orig);
}

#[cfg(test)]
mod tests {
    use crate::test_adder;
    use crate::test_multiplier;

    #[test]
    fn adder_basics() {
        test_adder(0, 0);
        test_adder(1, 0);
        test_adder(0, 1);
        test_adder(1, 1);
        test_adder(12, 42);
        test_adder(1, 1234);
        test_adder(24124, 2887979);
        test_adder(u32::MAX, 0);
        test_adder(u32::MAX - 1, 1);
        test_adder(u32::MAX - 123456, 123456);
    }

    #[test]
    fn adder_overflow() {
        test_adder(1, u32::MAX);
        test_adder(u32::MAX, 2);
        test_adder(2345, u32::MAX);
        test_adder(u32::MAX, u32::MAX);
        test_adder(u32::MAX/2, u32::MAX);
        test_adder(u32::MAX/2, u32::MAX/3);
    }

    #[test]
    fn multiplier_basics() {
        test_multiplier(0, 0);
        test_multiplier(3, 6);
        test_multiplier(9, 71);
        test_multiplier(187, 18);
        test_multiplier(3, 241341);
        test_multiplier(241341, 10);
        test_multiplier(0, u32::MAX);
        test_multiplier(u32::MAX, 0);
        test_multiplier(u32::MAX, 1);
        test_multiplier(1, u32::MAX);
    }

    #[test]
    fn multiplier_overflow() {
        test_multiplier(10, u32::MAX);
        test_multiplier(1000, u32::MAX);
        test_multiplier(u32::MAX, u32::MAX);
        test_multiplier(u32::MAX/2, u32::MAX/2);
    }
}
