use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

/// 并查集实现
#[derive(Default)]
pub struct Union {
  /// key: child
  /// value: parent
  p: HashMap<usize, usize>,
}

impl Union {
  pub fn new() -> Self {
    Self::default()
  }

  /// 初始化使用，插入数据到并查集中
  pub fn insert(&mut self, child: usize, parent: usize) {
    self.p.insert(child, parent);
  }

  /// 合并n1和n2两个节点所在的树
  pub fn union(&mut self, n1: usize, n2: usize) {
    let n1_root = self.root(n1);
    let n2_root = self.root(n2);
    self.insert(n1_root, n2_root);
  }

  /// 返回node节点所在树的根节点
  pub fn root(&mut self, mut node: usize) -> usize {
    let mut root = node;
    while self.p.get(&root) != Some(&root) {
      root = *self.p.get(&root).unwrap();
    }
    while self.p.get(&node) != Some(&node) {
      let x = node;
      node = *self.p.get(&node).unwrap();
      self.p.insert(x, root);
    }
    root
  }

  /// 返回并查集中树的数量
  pub fn len(&mut self) -> usize {
    let roots: Vec<usize> = self.p.keys().map(|&elem| elem).collect();
    let sets: HashSet<usize> = HashSet::from_iter(roots.into_iter().map(|elem| self.root(elem)));
    sets.len()
  }
}

impl Solution {
  pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    if is_connected.is_empty() {
      return 0;
    }
    let rows = is_connected.len();
    let cols = is_connected[0].len();

    let mut u = Union::new();
    for row in 0..rows {
      u.insert(row, row);
    }

    for row in 0..rows {
      for col in 0..cols {
        if is_connected[row][col] == 1 {
          u.union(row, col);
        }
      }
    }

    u.len() as i32
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let is_connected = vec![
      vec![1, 0, 0, 1],
      vec![0, 1, 1, 0],
      vec![0, 1, 1, 1],
      vec![1, 0, 1, 1],
    ];
    assert_eq!(Solution::find_circle_num(is_connected), 1);
  }
}
