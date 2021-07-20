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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
  fn new() -> Self {
    Codec {}
  }

  fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut result = Vec::new();
    Codec::serializeDFS(&root, &mut result);
    result.join(",")
  }

  fn serializeDFS(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<String>) {
    match root {
      Some(root) => {
        result.push(root.borrow().val.to_string());
        Codec::serializeDFS(&root.borrow().left, result);
        Codec::serializeDFS(&root.borrow().right, result);
      }
      None => {
        result.push("null".to_string());
      }
    }
  }

  fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() {
      return None;
    }
    let mut vals: VecDeque<Option<i32>> = data
      .split(',')
      .map(|x| {
        if x == "null" {
          None
        } else {
          Some(x.parse::<i32>().unwrap())
        }
      })
      .collect();
    Codec::deserialize_recursive(&mut vals)
  }

  fn deserialize_recursive(vals: &mut VecDeque<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() {
      return None;
    }

    if vals[0].is_none() {
      vals.pop_front();
      return None;
    }

    let mut node = TreeNode::new(vals.pop_front().unwrap().unwrap());
    node.left = Codec::deserialize_recursive(vals);
    node.right = Codec::deserialize_recursive(vals);
    Some(Rc::new(RefCell::new(node)))
  }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {}
}
