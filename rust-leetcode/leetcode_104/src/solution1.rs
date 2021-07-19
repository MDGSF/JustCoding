use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
      Some(root) => {
        let left = Solution::max_depth(root.borrow().left.clone());
        let right = Solution::max_depth(root.borrow().right.clone());
        std::cmp::max(left, right) + 1
      }
      None => 0,
    }
  }
}

pub struct Solution;

// Definition for a binary tree node.
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
