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
  pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    return Self::build(&preorder, &inorder);
  }

  fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
      return None;
    }
    let val = preorder[0];
    let idx = inorder.iter().position(|&x| x == val).unwrap();
    let mut node = TreeNode::new(val);
    node.left = Self::build(&preorder[1..(1 + idx)], &inorder[0..idx]);
    node.right = Self::build(&preorder[(1 + idx)..], &inorder[idx + 1..]);
    Some(Rc::new(RefCell::new(node)))
  }
}

pub struct Solution {}

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
