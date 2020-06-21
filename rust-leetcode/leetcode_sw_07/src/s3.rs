use std::collections::HashMap;

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
    let inorder_map: HashMap<i32, usize> =
      inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    return Self::build(0, &preorder, &inorder, &inorder_map);
  }

  fn build(
    rootidx: usize,
    preorder: &[i32],
    inorder: &[i32],
    inorder_map: &HashMap<i32, usize>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    if rootidx >= preorder.len() {
      return None;
    }
    let val = preorder[rootidx];
    let idx = inorder_map[&val];
    let mut node = TreeNode::new(val);
    println!(
      "val = {}, idx = {}, preorder = {:?}, inorder = {:?}",
      val, idx, preorder, inorder
    );
    node.left = Self::build(rootidx + 1, &preorder, &inorder, &inorder_map);
    node.right = Self::build(rootidx + 1 + idx, &preorder, &inorder, &inorder_map);
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
