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
  pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    Self::dfs(root, 0, &mut result);
    result
  }

  fn dfs(mut node: Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<Vec<i32>>) {
    if node.is_none() {
      return;
    }
    if result.len() < level + 1 {
      result.push(Vec::new());
    }
    result[level].push(node.as_ref().unwrap().borrow().val);
    Self::dfs(
      node.as_mut().unwrap().borrow_mut().left.take(),
      level + 1,
      result,
    );
    Self::dfs(
      node.as_mut().unwrap().borrow_mut().right.take(),
      level + 1,
      result,
    );
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
