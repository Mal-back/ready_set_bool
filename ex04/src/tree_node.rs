use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    operation::Operation,
};

#[derive(Debug)]
pub enum NodeType {
    Leaf(char),
    Node(Operation),
}

#[derive(Debug)]
pub struct TreeNode {
    pub node_type: NodeType,
    left_child: Option<Box<TreeNode>>,
    right_child: Option<Box<TreeNode>>,
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
