use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
  pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let directions = vec![
      vec![0, 1],  // 北
      vec![1, 0],  // 东
      vec![0, -1], // 南
      vec![-1, 0], // 西
    ];
    let mut di = 0;
    let mut x = 0;
    let mut y = 0;
    let mut result = 0;
    let obstacles: HashSet<(i32, i32)> =
      HashSet::from_iter(obstacles.iter().map(|point| (point[0], point[1])));
    for &cmd in commands.iter() {
      if cmd == -1 {
        di = (di + 1) % 4; // 右转90度
      } else if cmd == -2 {
        di = (di + 3) % 4; // 左转90度
      } else {
        for _i in 0..cmd {
          let new_x = x + directions[di][0];
          let new_y = y + directions[di][1];
          if !obstacles.contains(&(new_x, new_y)) {
            x = new_x;
            y = new_y;
            result = result.max(x * x + y * y);
          }
        }
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
  fn test() {
    assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
    assert_eq!(
      Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]),
      65
    );
  }
}
