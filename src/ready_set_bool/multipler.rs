use super::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;
    if b == 0 {
        return 0;
    }
        if b & 1 == 1 {
            res = adder(res, a);
        }
    return res + multiplier(a << 1, b >> 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adder_test_ok() {
        assert_eq!(multiplier(10, 5), 50);
        assert_eq!(multiplier(0, 15), 0);
        assert_eq!(multiplier(3, 48), 144);
        assert_eq!(multiplier(13, 0), 0);
    }
}
