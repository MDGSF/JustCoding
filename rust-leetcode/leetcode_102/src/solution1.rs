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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
  pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
      return Vec::new();
    }
    let mut result = Vec::new();
    let mut deque = VecDeque::new();
    deque.push_back(root);
    while !deque.is_empty() {
      let level_size = deque.len();
      let mut current_level = Vec::new();
      for _i in 0..level_size {
        let node = deque.pop_front().unwrap().unwrap();
        current_level.push(node.borrow().val);
        if node.borrow().left.is_some() {
          deque.push_back(node.borrow_mut().left.take());
        }
        if node.borrow().right.is_some() {
          deque.push_back(node.borrow_mut().right.take());
        }
      }
      result.push(current_level);
    }
    result
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
