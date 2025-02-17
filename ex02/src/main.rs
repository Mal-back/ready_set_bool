fn main() {
    println!("gray_code of 0 : {}", gray_code(0));
    println!("gray_code of 1 : {}", gray_code(1));
    println!("gray_code of 2 : {}", gray_code(2));
    println!("gray_code of 3 : {}", gray_code(3));
    println!("gray_code of 4 : {}", gray_code(4));
    println!("gray_code of 5 : {}", gray_code(5));
    println!("gray_code of 6 : {}", gray_code(6));
    println!("gray_code of 7 : {}", gray_code(7));
    println!("gray_code of 8 : {}", gray_code(8));
}

fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gray_code_test_ok() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
    }
}
