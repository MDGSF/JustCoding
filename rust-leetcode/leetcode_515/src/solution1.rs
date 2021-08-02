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
  pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
      return Vec::new();
    }
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
      let levelsize = queue.len();
      let mut max = std::i32::MIN;
      for _i in 0..levelsize {
        let node = queue.pop_front().unwrap().unwrap();
        if node.borrow().val > max {
          max = node.borrow().val;
        }
        if node.borrow().left.is_some() {
          queue.push_back(node.borrow_mut().left.take());
        }
        if node.borrow().right.is_some() {
          queue.push_back(node.borrow_mut().right.take());
        }
      }
      result.push(max);
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
