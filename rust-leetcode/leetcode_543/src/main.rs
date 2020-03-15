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
  pub fn depth(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
    match root {
      Some(node) => {
        let left = Solution::depth(node.clone().borrow().left.clone(), result);
        let right = Solution::depth(node.clone().borrow().right.clone(), result);
        *result = std::cmp::max(*result, left + right);
        std::cmp::max(left, right) + 1
      }
      None => 0,
    }
  }

  pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut result = 0;
    Solution::depth(root, &mut result);
    result
  }
}

struct Solution;

fn main() {
  println!("Hello, world!");
}
