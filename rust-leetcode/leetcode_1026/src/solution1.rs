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
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let max_value = root.as_ref().unwrap().borrow().val;
        let min_value = root.as_ref().unwrap().borrow().val;
        max(
            Solution::recursion(&root.as_ref().unwrap().borrow().left, max_value, min_value),
            Solution::recursion(&root.as_ref().unwrap().borrow().right, max_value, min_value),
        )
    }

    fn recursion(node: &Option<Rc<RefCell<TreeNode>>>, max_value: i32, min_value: i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let value = node.as_ref().unwrap().borrow().val;
        let max_value = max_value.max(value);
        let min_value = min_value.min(value);
        let cur = (max_value - min_value).abs();

        let sub = max(
            Solution::recursion(&node.as_ref().unwrap().borrow().left, max_value, min_value),
            Solution::recursion(&node.as_ref().unwrap().borrow().right, max_value, min_value),
        );

        max(cur, sub)
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
