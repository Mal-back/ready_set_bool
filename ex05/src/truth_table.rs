use crate::{
    error::{Error, Result},
    operation::Operation,
    tree_node::{NodeType, TreeNode},
};

#[derive(Debug)]
pub struct TruthTable {
    root: TreeNode,
}

impl TruthTable {
    pub fn build_from_str(input: &str) -> Result<Self> {
        let mut stack = vec![];
        for character in input.chars() {
            match character {
                character if "!&|^>=".contains(character) => {
                    let new_node = Self::build_new_node(&mut stack, character)?;
                    stack.push(new_node)
                }
                character if character.is_ascii_uppercase() => {
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

    pub fn turn_into_negation_normal_form(&mut self) {
        self.root.simplify_node();
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
