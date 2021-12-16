use std::convert::TryInto;

fn main() {
    test_adder(1234, 4321);
}

fn getbit(n: u32, i: u8) -> u8 {
    ((n >> i) & 1).try_into().unwrap()
}

fn setbit(n: u32, i: u8, bit: u8) -> u32 {
    if bit != 0 {
        n | (1 << i)
    } else if getbit(n, i) != 0 {
        n ^ (1 << i)
    } else {
        n
    }
}

fn adder(a: u32, b: u32) -> u32 {
    const BITLEN: u8 = 32;
    let mut bita: u8;
    let mut bitb: u8;
    let mut newbit: u8;
    let mut carry: u8 = 0;
    let mut result: u32 = 0;

    for i in 0..BITLEN {
        bita = getbit(a, i);
        bitb = getbit(b, i);
        newbit = carry;
        if bita ^ bitb != 0 {
            carry = newbit & 1;
            newbit = newbit ^ 1;
        } else {
            carry = bita & bitb;
        }
        result = setbit(result, i, newbit);
    }
    result
}

fn test_adder(left: u32, right: u32) {
    let mine: u32 = adder(left, right);
    let orig: u32 = left.wrapping_add(right);
    println!("{}+{}: mine = {}, orig = {}", left, right, mine, orig);
    assert_eq!(mine, orig);
}

#[cfg(test)]
mod tests {
    use crate::test_adder;

    #[test]
    fn adder_basics() {
        test_adder(12, 42);
        test_adder(1, 0);
        test_adder(0, 1);
        test_adder(1, 1);
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
}
