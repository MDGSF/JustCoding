use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    if root.is_none() {
      return result;
    }

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut r = root.clone();

    // 当前节点非空或栈非空时循环
    while r.is_some() || !stack.is_empty() {
      // 若当前节点非空，将当前节点入栈，并进入其左子树访问
      while let Some(node) = r {
        stack.push(node.clone());
        r = node.borrow().left.clone();
      }

      // 栈顶节点出栈，访问其节点值，并进入其右子树访问
      r = stack.pop();
      if let Some(node) = r {
        result.push(node.borrow().val);
        r = node.borrow().right.clone();
      }
    }

    result
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let input = vec![
      Some(1),
      Some(2),
      Some(3),
      Some(4),
      Some(5),
      Some(6),
      Some(7),
    ];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::inorder_traversal(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![4, 2, 5, 1, 6, 3, 7]);
  }

  #[test]
  fn test_2() {
    let input = vec![Some(1), None, Some(2), None, None, Some(3)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::inorder_traversal(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![1, 3, 2]);
  }
}
