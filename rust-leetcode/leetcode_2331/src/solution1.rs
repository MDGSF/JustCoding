// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::evaluate(&root)
    }

    fn evaluate(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if node.is_none() {
            return false;
        }

        let val = node.as_ref().unwrap().borrow().val;
        let left = &node.as_ref().unwrap().borrow().left;
        let right = &node.as_ref().unwrap().borrow().right;
        match val {
            0 => false,
            1 => true,
            2 => Self::evaluate(left) || Self::evaluate(right),
            3 => Self::evaluate(left) && Self::evaluate(right),
            _ => unreachable!(),
        }
    }
}

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
