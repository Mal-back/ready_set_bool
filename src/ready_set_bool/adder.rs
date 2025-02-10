pub fn adder(a: u32, b: u32) -> u32 {
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
    fn adder_test_ok() {
        assert_eq!(adder(10, 5), 15);
        assert_eq!(adder(0, 15), 15);
        assert_eq!(adder(3, 48), 51);
        assert_eq!(adder(13, 0), 13);
    }
}
