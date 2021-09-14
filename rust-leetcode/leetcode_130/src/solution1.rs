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
  pub fn solve(board: &mut Vec<Vec<char>>) {
    if board.is_empty() {
      return;
    }
    let rows = board.len();
    let cols = board[0].len();

    let mut u = Union::new();
    let dummy = rows * cols;
    u.insert(dummy, dummy);

    for row in 0..rows {
      for col in 0..cols {
        if board[row][col] == 'O' {
          u.insert(row * cols + col, row * cols + col);
        }
      }
    }

    let directions = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];
    for row in 0..rows {
      for col in 0..cols {
        if board[row][col] == 'O' {
          if Self::is_boundary(board, row, col) {
            u.union(row * cols + col, dummy);
          } else {
            let old_row = row as isize;
            let old_col = col as isize;
            for direction in directions.iter() {
              let new_row = old_row + direction[0];
              let new_col = old_col + direction[1];

              if new_row >= 0
                && new_row < rows as isize
                && new_col >= 0
                && new_col < cols as isize
                && board[new_row as usize][new_col as usize] == 'O'
              {
                u.union(row * cols + col, new_row as usize * cols + new_col as usize);
              }
            }
          }
        }
      }
    }

    for row in 0..rows {
      for col in 0..cols {
        if board[row][col] == 'O' {
          let p1 = u.root(row * cols + col);
          let p2 = u.root(dummy);
          if p1 == p2 {
            board[row][col] = 'O';
          } else {
            board[row][col] = 'X';
          }
        }
      }
    }
  }

  fn is_boundary(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
    row == 0 || row == board.len() - 1 || col == 0 || col == board[0].len() - 1
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut board = vec![
      vec!['X', 'X', 'X', 'X'],
      vec!['X', 'O', 'O', 'X'],
      vec!['X', 'X', 'O', 'X'],
      vec!['X', 'O', 'X', 'X'],
    ];

    Solution::solve(&mut board);

    let expected_result = vec![
      vec!['X', 'X', 'X', 'X'],
      vec!['X', 'X', 'X', 'X'],
      vec!['X', 'X', 'X', 'X'],
      vec!['X', 'O', 'X', 'X'],
    ];
    assert_eq!(board, expected_result);
  }
}
