use super::formula::Formula;

pub fn eval_formula(input: &str) -> bool {
    let res;
    match Formula::new(input) {
        Ok(mut formula) => res = formula.evaluate(),
        Err(e) => {
            println!("{e}");
            return false;
        }
    }
    match res {
        Ok(r) => return r,
        Err(e) => {
            println!("{e}");
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eval_formula_test_ok() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }
}
