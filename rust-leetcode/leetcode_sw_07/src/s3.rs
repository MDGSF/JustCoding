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
    return Self::build(
      0,
      preorder.len(),
      0,
      inorder.len(),
      &preorder,
      &inorder,
      &inorder_map,
    );
  }

  fn build(
    pre_start_idx: usize,
    pre_len: usize,
    in_start_idx: usize,
    in_len: usize,
    preorder: &[i32],
    inorder: &[i32],
    inorder_map: &HashMap<i32, usize>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    //println!(
    //  "\npre_start_idx = {}, pre_len= {}, in_start_idx = {}, in_len = {}",
    //  pre_start_idx, pre_len, in_start_idx, in_len
    //);

    if pre_len == 0 || in_len == 0 {
      return None;
    }

    let val = preorder[pre_start_idx];
    let idx = inorder_map[&val];
    let leftlength = idx - in_start_idx;
    let rightlength = pre_len - 1 - leftlength;
    let mut node = TreeNode::new(val);
    //println!(
    //  "val = {}, idx = {}, leftlength = {}, rightlength = {}, preorder = {:?}, inorder = {:?}",
    //  val, idx, leftlength, rightlength, preorder, inorder
    //);
    node.left = Self::build(
      pre_start_idx + 1,
      leftlength,
      in_start_idx,
      leftlength,
      &preorder,
      &inorder,
      &inorder_map,
    );
    node.right = Self::build(
      pre_start_idx + 1 + leftlength,
      rightlength,
      in_start_idx + 1 + leftlength,
      rightlength,
      &preorder,
      &inorder,
      &inorder_map,
    );
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
