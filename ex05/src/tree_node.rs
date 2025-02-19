use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    operation::Operation,
};

#[derive(Debug, Clone)]
pub enum NodeType {
    Leaf(char),
    Node(Operation),
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub node_type: NodeType,
    left_child: Option<Box<TreeNode>>,
    right_child: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn build(
        node_type: NodeType,
        right_child: Option<Box<TreeNode>>,
        left_child: Option<Box<TreeNode>>,
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
            NodeType::Node(Operation::Xor) => self.simplify_xor(),
            NodeType::Node(Operation::IfThen) => self.simplify_if_then(),
            NodeType::Node(Operation::Equality) => self.simplify_equality(),
        }
    }

    pub fn print_rpn_op_from_tree(&self) {
        if let Some(left_child) = &self.left_child {
            left_child.print_rpn_op_from_tree();
        }
        if let Some(right_child) = &self.right_child {
            right_child.print_rpn_op_from_tree();
        }
        match &self.node_type {
            NodeType::Node(op) => print!("{op}"),
            NodeType::Leaf(var) => print!("{var}"),
        }
    }

    fn simplify_xor(&mut self) {
        let origin_left_child = self.left_child.take();
        let origin_right_child = self.right_child.take();

        let negate_left_child = TreeNode::build(
            NodeType::Node(Operation::Not),
            None,
            origin_left_child.clone(),
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        let negate_right_child = TreeNode::build(
            NodeType::Node(Operation::Not),
            None,
            origin_right_child.clone(),
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        let new_left_child = TreeNode::build(
            NodeType::Node(Operation::And),
            origin_left_child,
            Some(negate_right_child).map(Box::new),
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        let new_right_child = TreeNode::build(
            NodeType::Node(Operation::And),
            origin_right_child,
            Some(negate_left_child).map(Box::new),
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        self.left_child = Some(new_left_child).map(Box::new);
        self.right_child = Some(new_right_child).map(Box::new);
        self.node_type = NodeType::Node(Operation::Or);
    }

    fn simplify_if_then(&mut self) {
        let origin_left_child = self.left_child.take();

        let negate_left_child = TreeNode::build(
            NodeType::Node(Operation::Not),
            None,
            origin_left_child.clone(),
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        self.left_child = Some(negate_left_child).map(Box::new);
        self.node_type = NodeType::Node(Operation::Or);
    }

    fn simplify_equality(&mut self) {
        let origin_left_child = self.left_child.take();
        let origin_right_child = self.right_child.take();

        let mut new_left_child = TreeNode::build(
            NodeType::Node(Operation::IfThen),
            origin_left_child.clone(),
            origin_right_child.clone(),
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        let mut new_right_child = TreeNode::build(
            NodeType::Node(Operation::IfThen),
            origin_right_child.clone(),
            origin_left_child.clone(),
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        new_left_child.simplify_if_then();
        new_right_child.simplify_if_then();

        self.left_child = Some(new_left_child).map(Box::new);
        self.right_child = Some(new_right_child).map(Box::new);
        self.node_type = NodeType::Node(Operation::And);
    }
}
