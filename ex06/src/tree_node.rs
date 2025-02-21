use crate::{
    error::{Error, Result},
    operation::Operation,
};

#[derive(Debug, Clone, PartialEq)]
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

impl NodeType {
    pub fn is_leaf(&self) -> bool {
        match self {
            NodeType::Node(_) => false,
            NodeType::Leaf(_) => true,
        }
    }
    pub fn is_negation(&self) -> bool {
        match self {
            NodeType::Node(Operation::Not) => true,
            _ => false,
        }
    }
    pub fn is_or(&self) -> bool {
        match self {
            NodeType::Node(Operation::Or) => true,
            _ => false,
        }
    }

    pub fn is_and(&self) -> bool {
        match self {
            NodeType::Node(Operation::And) => true,
            _ => false,
        }
    }
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

    pub fn push_negation(&mut self) {
        match self.node_type {
            NodeType::Leaf(_) => return,
            NodeType::Node(Operation::Not)
                if self
                    .left_child
                    .as_ref()
                    .expect("We know a left_child is present a this point")
                    .node_type
                    .is_leaf() =>
            {
                return
            }
            NodeType::Node(Operation::Not) => {
                self.handle_negation_node();
                self.push_negation();
            }
            NodeType::Node(Operation::And) | NodeType::Node(Operation::Or) => {}
            _ => panic!("Other node types should not exists at this point !"),
        }

        if let Some(left_child) = &mut self.left_child {
            left_child.push_negation();
        }
        if let Some(right_child) = &mut self.right_child {
            right_child.push_negation();
        }
    }

    pub fn convert_to_conjuctive_normal_form(&mut self, modifed: &mut bool) {
        if self.node_type.is_leaf() || self.node_type.is_negation() {
            return;
        }

        if self.node_type.is_or() {
            let right_child = self
                .right_child
                .as_ref()
                .expect("We know a right child is present at this point");
            let left_child = self
                .left_child
                .as_ref()
                .expect("We know a right child is present at this point");

            if left_child.node_type.is_and()
                || (left_child.node_type.is_and() && right_child.node_type.is_and())
            {
                // do right child And permutation
                self.handle_left_child_is_and();
                *modifed = true;
            } else if right_child.node_type.is_and() {
                // do right_child and permutation
                self.handle_right_child_is_and();
                *modifed = true;
            }
        }

        self.left_child
            .as_mut()
            .expect("We do have a left child at this point")
            .convert_to_conjuctive_normal_form(modifed);
        self.right_child
            .as_mut()
            .expect("We do have a left child at this point")
            .convert_to_conjuctive_normal_form(modifed);
    }

    fn handle_left_child_is_and(&mut self) {
        let left_grand_children = self
            .left_child
            .as_ref()
            .expect("We know he's here")
            .left_child
            .as_ref()
            .expect("We know he's here")
            .clone();
        let right_grand_children = self
            .left_child
            .as_ref()
            .expect("We know he's here")
            .right_child
            .as_ref()
            .expect("We know he's here")
            .clone();
        let left_children = self.left_child.as_mut().expect("We know he's here");
        let right_children = self.right_child.as_mut().expect("We know he's here");

        self.node_type = NodeType::Node(Operation::And);

        left_children.left_child = Some(left_grand_children);
        left_children.right_child = Some(right_children.clone());
        right_children.left_child = Some(right_grand_children);
        right_children.right_child = Some(right_children.clone());

        left_children.node_type = NodeType::Node(Operation::Or);
        right_children.node_type = NodeType::Node(Operation::Or);
    }
    fn handle_right_child_is_and(&mut self) {
        let left_grand_children = self
            .right_child
            .as_ref()
            .expect("We know he's here")
            .left_child
            .as_ref()
            .expect("We know he's here")
            .clone();
        let right_grand_children = self
            .right_child
            .as_ref()
            .expect("We know he's here")
            .right_child
            .as_ref()
            .expect("We know he's here")
            .clone();
        let left_children = self.left_child.as_mut().expect("We know he's here");
        let right_children = self.right_child.as_mut().expect("We know he's here");

        self.node_type = NodeType::Node(Operation::And);

        left_children.left_child = Some(left_grand_children);
        left_children.right_child = Some(right_children.clone());
        right_children.left_child = Some(right_grand_children);
        right_children.right_child = Some(right_children.clone());

        left_children.node_type = NodeType::Node(Operation::Or);
        right_children.node_type = NodeType::Node(Operation::Or);
    }

    fn handle_negation_node(&mut self) {
        match self
            .left_child
            .as_ref()
            .expect("Negation Node should have a left_child")
            .node_type
        {
            NodeType::Leaf(_) => {}
            NodeType::Node(Operation::Not) => {
                let new_node_type = self
                    .left_child
                    .as_ref()
                    .unwrap()
                    .left_child
                    .as_ref()
                    .unwrap()
                    .node_type
                    .clone();

                let new_left_child = self
                    .left_child
                    .as_ref()
                    .unwrap()
                    .left_child
                    .as_ref()
                    .unwrap()
                    .left_child
                    .clone();

                let new_right_child = self
                    .left_child
                    .as_ref()
                    .unwrap()
                    .left_child
                    .as_ref()
                    .unwrap()
                    .right_child
                    .clone();

                self.node_type = new_node_type;
                self.left_child = new_left_child;
                self.right_child = new_right_child;
                return;
                // get the grandchild op type and child and set them to self
            }
            NodeType::Node(Operation::Or) => {
                let child = self.left_child.as_ref().unwrap().clone();
                let new_left_child = TreeNode::build(
                    NodeType::Node(Operation::Not),
                    None,
                    Some(child.left_child.as_ref().unwrap().clone()),
                )
                .unwrap();
                let new_right_child = TreeNode::build(
                    NodeType::Node(Operation::Not),
                    None,
                    Some(child.right_child.as_ref().unwrap().clone()),
                )
                .unwrap();
                self.node_type = NodeType::Node(Operation::And);
                self.right_child = Some(new_right_child).map(Box::new);
                self.left_child = Some(new_left_child).map(Box::new);
                // do not or simplification
            }
            NodeType::Node(Operation::And) => {
                let child = self.left_child.as_ref().unwrap().clone();
                let new_left_child = TreeNode::build(
                    NodeType::Node(Operation::Not),
                    None,
                    Some(child.left_child.as_ref().unwrap().clone()),
                )
                .unwrap();
                let new_right_child = TreeNode::build(
                    NodeType::Node(Operation::Not),
                    None,
                    Some(child.right_child.as_ref().unwrap().clone()),
                )
                .unwrap();
                self.node_type = NodeType::Node(Operation::Or);
                self.right_child = Some(new_right_child).map(Box::new);
                self.left_child = Some(new_left_child).map(Box::new);
                // do not and simplification
            }
            _ => panic!("Other node should not be seen here"),
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
        let not_left_child = TreeNode::build(
            NodeType::Node(Operation::Not),
            None,
            origin_left_child.clone(),
        )
        .unwrap();
        let not_right_child = TreeNode::build(
            NodeType::Node(Operation::Not),
            None,
            origin_right_child.clone(),
        )
        .unwrap();

        let new_left_child = TreeNode::build(
            NodeType::Node(Operation::And),
            origin_left_child,
            origin_right_child,
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        let new_right_child = TreeNode::build(
            NodeType::Node(Operation::And),
            Some(not_right_child).map(Box::new),
            Some(not_left_child).map(Box::new),
        )
        .expect("Nothing should fail at this point, we know the node will be valid");

        self.left_child = Some(new_left_child).map(Box::new);
        self.right_child = Some(new_right_child).map(Box::new);
        self.node_type = NodeType::Node(Operation::Or);
    }
}
