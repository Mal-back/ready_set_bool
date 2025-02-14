use std::collections::hash_map;

use crate::ready_set_bool::Error;

use super::{operation::Operation, Result};

#[derive(Debug)]
enum NodeType {
    Leaf(char),
    Node(Operation),
}

#[derive(Debug)]
struct TreeNode {
    node_type: NodeType,
    left_child: Option<Box<TreeNode>>,
    right_child: Option<Box<TreeNode>>,
}

#[derive(Debug)]
pub struct TruthTable {
    root: TreeNode,
    expression_variables: Vec<char>,
}

impl TruthTable {
    pub fn build_from_str(input: &str) -> Result<Self> {
        let mut stack = vec![];
        let mut expression_variables = Vec::with_capacity(26);
        for character in input.chars() {
            match character {
                character if "!&|^>=".contains(character) => {
                    let new_node = Self::build_new_node(&mut stack, character)?;
                    stack.push(new_node)
                }
                character if character.is_ascii_uppercase() => {
                    if !expression_variables.contains(&character) {
                        expression_variables.insert(0, character);
                    }
                    let new_node = TreeNode::build(NodeType::Leaf(character), None, None)?;
                    stack.push(new_node);
                }
                _ => return Err(Error::InvalidFormulaSyntax),
            }
        }
        if stack.len() != 1 {
            Err(Error::InvalidFormulaGrammar)
        } else {
            Ok(Self {
                root: stack.pop().expect("This should not be None at this point"),
                expression_variables,
            })
        }
    }

    fn build_new_node(stack: &mut Vec<TreeNode>, symbol: char) -> Result<TreeNode> {
        let (operation, number_of_childs) = Operation::new(symbol);
        let right_child = if number_of_childs == 2 {
            stack.pop()
        } else {
            None
        };
        let left_child = stack.pop();
        TreeNode::build(NodeType::Node(operation), right_child, left_child)
    }

    pub fn compute_truth_table(&self) {
        let number_of_variables = self.expression_variables.len();
        let number_of_combination = 2u32.pow(number_of_variables as u32);

        for variable_values in 0..number_of_combination {}
    }
}

impl TreeNode {
    pub fn build(
        node_type: NodeType,
        right_child: Option<TreeNode>,
        left_child: Option<TreeNode>,
    ) -> Result<Self> {
        match node_type {
            NodeType::Leaf(_) => {}
            NodeType::Node(Operation::Not) => {
                if left_child.is_none() {
                    return Err(Error::InvalidFormulaGrammar);
                }
            }
            NodeType::Node(_) => {
                if left_child.is_none() || right_child.is_none() {
                    return Err(Error::InvalidFormulaGrammar);
                }
            }
        }
        let left_child = left_child.map(Box::new);
        let right_child = right_child.map(Box::new);
        Ok(Self {
            node_type,
            left_child,
            right_child,
        })
    }

    pub fn get_leaf_value(&self, variables_values: u32, expression_variables: Vec<char>) -> bool {
        match self.node_type {
            NodeType::Node(_) => panic!("This should only be called on a leaf"),
            NodeType::Leaf(variable) => {
                self.get_concrete_value(variable, variables_values, expression_variables)
            }
        }
    }

    fn get_concrete_value(
        &self,
        variable: char,
        variables_values: u32,
        expression_variables: Vec<char>,
    ) -> bool {
        let variable_index = expression_variables
            .iter()
            .position(|c| *c == variable)
            .expect("Variable should be present in the known variables");

        self.extract_current_value(variable_index + 1, variables_values)
    }

    fn extract_current_value(&self, index: usize, variables_values: u32) -> bool {
        let mask = 1 << index;

        let bit_of_interest = variables_values & mask;

        if bit_of_interest != 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_truth_table_ok() {
        let tree = TruthTable::build_from_str("AB&");
        assert!(tree.is_ok());
        let tree = TruthTable::build_from_str("ABCD||=");
        assert!(tree.is_ok());
        assert_eq!(tree.unwrap().expression_variables, vec!['D', 'C', 'B', 'A']);
        let tree = TruthTable::build_from_str("A!");
        assert!(tree.is_ok());
    }

    #[test]
    fn truth_table_err_invalid_characters() {
        let tree = TruthTable::build_from_str("EAu&");
        println!("{tree:?}");
        assert!(matches!(tree, Err(Error::InvalidFormulaSyntax)));
    }

    #[test]
    fn truth_table_err_invalid_grammar() {
        let tree = TruthTable::build_from_str("AAA&");
        println!("{tree:?}");
        assert!(matches!(tree, Err(Error::InvalidFormulaGrammar)));
        let tree = TruthTable::build_from_str("AB&CD&");
        println!("{tree:?}");
        assert!(matches!(tree, Err(Error::InvalidFormulaGrammar)));
    }
}
