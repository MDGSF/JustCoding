use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
  pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    if is_connected.is_empty() {
      return 0;
    }
    let rows = is_connected.len();
    let cols = is_connected[0].len();

    let mut p = HashMap::new();
    for row in 0..rows {
      p.insert(row, row);
    }

    for row in 0..rows {
      for col in 0..cols {
        if is_connected[row][col] == 1 {
          Self::union(&mut p, row, col);
        }
      }
    }

    let roots: Vec<usize> = p.keys().map(|&elem| elem).collect();
    let sets: HashSet<usize> =
      HashSet::from_iter(roots.into_iter().map(|elem| Self::parent(&mut p, elem)));
    sets.len() as i32
  }

  fn union(p: &mut HashMap<usize, usize>, i: usize, j: usize) {
    let p1 = Self::parent(p, i);
    let p2 = Self::parent(p, j);
    p.insert(p1, p2);
  }

  fn parent(p: &mut HashMap<usize, usize>, mut i: usize) -> usize {
    let mut root = i;
    while p.get(&root) != Some(&root) {
      root = *p.get(&root).unwrap();
    }
    while p.get(&i) != Some(&i) {
      let x = i;
      i = *p.get(&i).unwrap();
      p.insert(x, root);
    }
    root
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
