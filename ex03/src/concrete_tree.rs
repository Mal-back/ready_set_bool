use crate::{
    error::{Error, Result},
    operation::Operation,
};

enum NodeType {
    Leaf(bool),
    Node(Operation),
}

struct FormulaNode {
    node_type: NodeType,
    left_child: Option<Box<FormulaNode>>,
    right_child: Option<Box<FormulaNode>>,
}

pub struct ConcreteFormulaTree {
    root: FormulaNode,
}

impl ConcreteFormulaTree {
    pub fn build(formula: &str) -> Result<Self> {
        let mut stack = vec![];
        for character in formula.chars() {
            match character {
                '0' => stack.push(FormulaNode {
                    node_type: NodeType::Leaf(false),
                    left_child: None,
                    right_child: None,
                }),
                '1' => stack.push(FormulaNode {
                    node_type: NodeType::Leaf(true),
                    left_child: None,
                    right_child: None,
                }),
                '!' => Self::build_negation_node(&mut stack)?,
                '&' => Self::build_operation_node(Operation::And, &mut stack)?,
                '|' => Self::build_operation_node(Operation::Or, &mut stack)?,
                '^' => Self::build_operation_node(Operation::Xor, &mut stack)?,
                '>' => Self::build_operation_node(Operation::IfThen, &mut stack)?,
                '=' => Self::build_operation_node(Operation::Equality, &mut stack)?,
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

    fn build_negation_node(stack: &mut Vec<FormulaNode>) -> Result<()> {
        if stack.len() < 1 {
            return Err(Error::InvalidFormulaGrammar);
        }
        let child = stack
            .pop()
            .expect("Stack should be assured to have 1 element at this point");
        stack.push(FormulaNode {
            node_type: NodeType::Node(Operation::Not),
            left_child: Some(Box::new(child)),
            right_child: None,
        });
        Ok(())
    }

    fn build_operation_node(operation: Operation, stack: &mut Vec<FormulaNode>) -> Result<()> {
        if stack.len() < 2 {
            return Err(Error::InvalidFormulaGrammar);
        }
        let right_child = stack
            .pop()
            .expect("Stack should be assured to have 2 elements at this point");
        let left_child = stack
            .pop()
            .expect("Stack should be assured to have 2 elements at this point");
        stack.push(FormulaNode {
            node_type: NodeType::Node(operation),
            right_child: Some(Box::new(right_child)),
            left_child: Some(Box::new(left_child)),
        });
        Ok(())
    }

    pub fn resolve_tree(self) -> bool {
        Self::resolve_node(&self.root)
    }

    fn resolve_node(node: &FormulaNode) -> bool {
        let right_child_value;
        let left_child_value;
        if let Some(right_child) = &node.right_child {
            right_child_value = match right_child.node_type {
                NodeType::Node(_) => Self::resolve_node(&right_child),
                NodeType::Leaf(value) => value,
            }
        } else {
            right_child_value = true;
        }
        if let Some(left_child) = &node.left_child {
            left_child_value = match left_child.node_type {
                NodeType::Node(_) => Self::resolve_node(&left_child),
                NodeType::Leaf(value) => value,
            }
        } else {
            panic!("This node should never be None");
        }

        let operation_closure = match &node.node_type {
            NodeType::Node(operation) => operation.get_operation_closure(),
            _ => panic!("resolve_node should never be called on a leaf node"),
        };

        operation_closure(left_child_value, right_child_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_tree_ok() {
        let tree = ConcreteFormulaTree::build("00&");
        assert!(tree.is_ok());
        let tree = ConcreteFormulaTree::build("1011||=");
        assert!(tree.is_ok());
    }

    #[test]
    fn eval_formula_err_invalid_characters() {
        let tree = ConcreteFormulaTree::build("00u&");
        assert!(matches!(tree, Err(Error::InvalidFormulaSyntax)));
    }

    #[test]
    fn eval_formula_err_invalid_grammar() {
        let tree = ConcreteFormulaTree::build("000&");
        assert!(matches!(tree, Err(Error::InvalidFormulaGrammar)));
        let tree = ConcreteFormulaTree::build("00&11&");
        assert!(matches!(tree, Err(Error::InvalidFormulaGrammar)));
    }
}
