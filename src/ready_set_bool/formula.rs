use super::{Error, Result};

pub struct Formula<'a> {
    input: &'a str,
    stack: Vec<bool>,
}

impl<'a> Formula<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let sanity_closure = |c| {
            c != '0'
                && c != '1'
                && c != '!'
                && c != '&'
                && c != '|'
                && c != '^'
                && c != '>'
                && c != '='
        };
        if input.chars().any(sanity_closure) {
            Err(Error::InvalidFormulaSyntax)
        } else {
            Ok(Self {
                input,
                stack: vec![],
            })
        }
    }
    pub fn evaluate(&mut self) -> Result<bool> {
        for characters in self.input.chars() {
            match characters {
                '0' => self.stack.push(false),
                '1' => self.stack.push(true),
                '!' => self.do_negation()?,
                '&' => self.do_op(|first, second| first & second)?,
                '|' => self.do_op(|first, second| first | second)?,
                '^' => self.do_op(|first, second| first ^ second)?,
                '>' => self.do_op(|first, second| {
                    if !first || (first && second) {
                        true
                    } else {
                        false
                    }
                })?,
                '=' => self.do_op(|first, second| first == second)?,
                _ => panic!("Invalid character should have been seen at Formula initialisation"),
            }
        }
        if self.stack.len() != 1 {
            Err(Error::InvalidFormulaResult)
        } else {
            Ok(self.stack.pop().expect("This should not be None"))
        }
    }

    fn do_op<F>(&mut self, op: F) -> Result<()>
    where
        F: FnOnce(bool, bool) -> bool,
    {
        if self.stack.len() < 2 {
            return Err(Error::InvalidFormulaGrammar);
        }
        let second = self
            .stack
            .pop()
            .expect("Stack len should be at least two at this point");
        let first = self
            .stack
            .pop()
            .expect("Stack len should be at least two at this point");
        self.stack.push(op(first, second));
        Ok(())
    }

    fn do_negation(&mut self) -> Result<()> {
        self.stack.push(true);
        self.do_op(|first, second| first ^ second)?;
        Ok(())
    }
}
