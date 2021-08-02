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
  pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack = std::collections::LinkedList::new();
    stack.push_back(root.clone());
    let mut res = vec![];
    while !stack.is_empty() {
      let mut values = vec![];
      for _ in 0..stack.len() {
        let front = stack.pop_front().unwrap();
        if let Some(node) = front {
          values.push(node.borrow().val);
          if node.borrow().right.is_some() {
            stack.push_back(node.borrow().right.clone());
          }
          if node.borrow().left.is_some() {
            stack.push_back(node.borrow().left.clone());
          }
        }
      }
      if values.len() > 0 {
        res.push(values.into_iter().max().unwrap());
      }
    }
    return res;
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
