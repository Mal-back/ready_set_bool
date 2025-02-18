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

    pub fn simplify_node(&mut self) {
        if let Some(left_child) = &mut self.left_child {
            left_child.simplify_node();
        }
        if let Some(right_child) = &mut self.right_child {
            right_child.simplify_node();
        }

        match self.node_type {
            NodeType::Leaf(_) => {}
            NodeType::Node(Operation::And)
            | NodeType::Node(Operation::Or)
            | NodeType::Node(Operation::Not) => {}
            NodeType::Node(Operation::Xor) => todo!(),
            NodeType::Node(Operation::IfThen) => todo!(),
            NodeType::Node(Operation::Equality) => todo!(),
        }
    }

    pub fn simplify_xor(&mut self) {
        let origin_left_child = self.left_child.take();
        let origin_right_child = self.right_child.take();
    }
}
