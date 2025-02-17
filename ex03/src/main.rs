use concrete_tree::ConcreteFormulaTree;

mod concrete_tree;
mod error;
mod operation;

fn main() {
    println!("Resolving 10& : {}", eval_formula("10&"));
    println!("Resolving 10| : {}", eval_formula("10|"));
    println!("Resolving 11> : {}", eval_formula("11>"));
    println!("Resolving 10= : {}", eval_formula("10="));
    println!("Resolving 1011||= : {}", eval_formula("1011||="));
    println!("Resolving 1! : {}", eval_formula("1!"));
    println!("Resolving 01> : {}", eval_formula("01>"));
}

pub fn eval_formula(input: &str) -> bool {
    let formula = match ConcreteFormulaTree::build(input) {
        Ok(formula) => formula,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    formula.resolve_tree()
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
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("01>"), true);
    }

    #[test]
    fn eval_formula_err_invalid_characters() {
        assert_eq!(eval_formula("01u"), false);
    }

    #[test]
    fn eval_formula_err_invalid_grammar() {
        assert_eq!(eval_formula("01|&"), false);
    }

    #[test]
    fn eval_formula_err_invalid_result() {
        assert_eq!(eval_formula("010001|&"), false);
    }
}
