fn main() {
    println!("Hello, world!");
}

fn getbit<T: >(n: T, i: u8) -> T {
    (n >> i) & 1
}

fn setbit<T: num::PrimInt>(n: T, i: u8, bit: u8) -> T {
    if bit != 0 {
        n | (1 << i)
    } else if getbit(n, i) != 0 {
        n ^ (1 << i)
    } else {
        n
    }
}

fn adder(a: u32, b: u32) -> u32 {
    const BITLEN: u8 = 31;
    let mut bita: u8;
    let mut bitb: u8;
    let mut newbit: u8;
    let mut result: u32;
    let mut carry: u8 = 0;

    for i in 0..BITLEN {
        bita = getbit(a, i);
        bitb = getbit(b, i);
        newbit = carry;
        carry = 0;
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
