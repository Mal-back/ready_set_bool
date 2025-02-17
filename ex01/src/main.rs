fn main() {
    println!("10 * 5 : {}", multiplier(10, 5));
    println!("0 * 15 : {}", multiplier(0, 15));
    println!("3 * 48 : {}", multiplier(3, 48));
    println!("13 * 0 : {}", multiplier(13, 0));
}

fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;
    if b == 0 {
        return 0;
    }
    if b & 1 == 1 {
        res = adder(res, a);
    }
    return res + multiplier(a << 1, b >> 1);
}

fn adder(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    let carry = (a & b) << 1;

    adder(a ^ b, carry)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multiplier_test_ok() {
        assert_eq!(multiplier(10, 5), 50);
        assert_eq!(multiplier(0, 15), 0);
        assert_eq!(multiplier(3, 48), 144);
        assert_eq!(multiplier(13, 0), 0);
    }
}
