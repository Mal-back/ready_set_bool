use std::collections::{hash_map, HashMap};

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

        self.print_truth_table_header();

        for variables_values in 0..number_of_combination {
            let value_map = self.compute_values_map(&variables_values);
            let result = self.root.resolve_node(&value_map);
            self.print_truth_table_line(&value_map, result);
        }
    }

    fn compute_values_map(&self, variables_values: &u32) -> HashMap<&char, bool> {
        let mut current_map = HashMap::new();
        for (index, variable) in self.expression_variables.iter().enumerate() {
            let variable_value = self.get_concrete_value(variables_values, index);
            current_map.insert(variable, variable_value);
        }
        current_map
    }

    fn get_concrete_value(&self, variables_values: &u32, index: usize) -> bool {
        let mask = 1 << index;

        let bit_of_interest = variables_values & mask;

        if bit_of_interest != 0 {
            true
        } else {
            false
        }
    }

    fn print_truth_table_header(&self) {
        for variable in self.expression_variables.iter().rev() {
            print!("| {variable} ");
        }
        println!("| = |");
        for _ in 0..self.expression_variables.len() {
            print!("|---")
        }
        println!("|---|");
    }

    fn print_truth_table_line(&self, value_map: &HashMap<&char, bool>, result: bool) {
        for variable in self.expression_variables.iter().rev() {
            print!(
                "| {} ",
                *value_map
                    .get(variable)
                    .expect("Variable should be present in the map") as u32
            );
        }
        println!("| {} |", result as u32);
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

    pub fn resolve_node(&self, value_map: &HashMap<&char, bool>) -> bool {
        let operation_closure = match &self.node_type {
            NodeType::Leaf(_) => panic!("resolve node should never be call on a leaf"),
            NodeType::Node(operation) => operation.get_operation_closure(),
        };
        let left_value;
        let right_value;
        if let Some(left_child) = &self.left_child {
            left_value = match left_child.node_type {
                NodeType::Leaf(var) => *value_map
                    .get(&var)
                    .expect("All variables should be in the map"),
                NodeType::Node(_) => left_child.resolve_node(value_map),
            }
        } else {
            panic!("A left_child should be present at this point");
        }
        if let Some(right_child) = &self.right_child {
            right_value = match right_child.node_type {
                NodeType::Leaf(var) => *value_map
                    .get(&var)
                    .expect("All variables should be in the map"),
                NodeType::Node(_) => right_child.resolve_node(value_map),
            }
        } else {
            right_value = match self.node_type {
                NodeType::Node(Operation::Not) => false,
                _ => panic!("A right_child should be present at this point"),
            }
        }
        operation_closure(left_value, right_value)
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
