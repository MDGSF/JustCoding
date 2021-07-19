use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    Solution::is_valid(root, None, None)
  }

  fn is_valid(root: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
    match root {
      Some(root) => {
        let root = root.borrow();
        if let Some(min) = min {
          if root.val <= min {
            return false;
          }
        }
        if let Some(max) = max {
          if root.val >= max {
            return false;
          }
        }
        Solution::is_valid(root.left.clone(), min, Some(root.val))
          && Solution::is_valid(root.right.clone(), Some(root.val), max)
      }
      None => true,
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
