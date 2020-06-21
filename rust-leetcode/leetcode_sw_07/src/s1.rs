
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
      if preorder.is_empty() || inorder.is_empty() {
        return None;
      }
      let val = preorder[0];
      let idx = inorder.iter().position(|&x| x == val);
      let idx = match idx {
        Some(idx) => idx,
        None => return None,
      };
      let mut node = TreeNode::new(val);
      node.left = Self::build_tree((&preorder[1..(1+idx)]).to_vec(), (&inorder[0..idx]).to_vec());
      node.right = Self::build_tree((&preorder[(1+idx)..]).to_vec(), (&inorder[idx+1..]).to_vec());
      Some(Rc::new(RefCell::new(node)))
    }
}

struct Solution{}

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
      right: None
    }
  }
}
