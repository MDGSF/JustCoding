use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl From<&[i32]> for Point {
  fn from(p: &[i32]) -> Self {
    Self(p[0], p[1])
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self(self.0 + other.0, self.1 + other.1)
  }
}

const ROWS: i32 = 1000000;
const COLS: i32 = 1000000;

impl Solution {
  pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
    let blocked: HashSet<Point> =
      HashSet::from_iter(blocked.iter().map(|x| Point::from(&x[..])).clone());
    let source_is_blocked = Solution::is_blocked(&blocked, source[..].into(), target[..].into());
    let target_is_blocked = Solution::is_blocked(&blocked, target[..].into(), source[..].into());
    !source_is_blocked && !target_is_blocked
  }

  fn is_blocked(blocked: &HashSet<Point>, start: Point, peer: Point) -> bool {
    let tresold_level = blocked.len() * blocked.len() / 2;
    let directions = vec![Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut node_count = 1;
    while let Some(node) = queue.pop_front() {
      if node_count > tresold_level {
        return false;
      }
      if node == peer {
        return false;
      }

      for &direction in directions.iter() {
        let new_node = node + direction;
        if new_node.0 >= 0
          && new_node.0 < ROWS
          && new_node.1 >= 0
          && new_node.1 < COLS
          && !blocked.contains(&new_node)
          && !visited.contains(&new_node)
        {
          visited.insert(new_node);
          queue.push_back(new_node);
          node_count += 1;
        }
      }
    }
    true
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let blocked = vec![vec![0, 1], vec![1, 0]];
    let source = vec![0, 0];
    let target = vec![0, 2];
    let result = Solution::is_escape_possible(blocked, source, target);
    assert_eq!(result, false);
  }

  #[test]
  fn test2() {
    let blocked = vec![];
    let source = vec![0, 0];
    let target = vec![999999, 999999];
    let result = Solution::is_escape_possible(blocked, source, target);
    assert_eq!(result, true);
  }

  #[test]
  fn test3() {
    let blocked = vec![vec![0, 3], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3]];
    let source = vec![0, 0];
    let target = vec![0, 2];
    let result = Solution::is_escape_possible(blocked, source, target);
    assert_eq!(result, true);
  }
}
