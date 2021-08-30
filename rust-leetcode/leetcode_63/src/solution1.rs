impl Solution {
  pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let rows = obstacle_grid.len();
    let cols = obstacle_grid[0].len();

    let mut dp = vec![vec![0; cols]; rows];

    for row in 0..rows {
      if obstacle_grid[row][0] == 1 {
        break;
      } else {
        dp[row][0] = 1;
      }
    }

    for col in 0..cols {
      if obstacle_grid[0][col] == 1 {
        break;
      } else {
        dp[0][col] = 1;
      }
    }

    for row in 1..rows {
      for col in 1..cols {
        if obstacle_grid[row][col] == 1 {
          dp[row][col] = 0;
        } else {
          dp[row][col] = dp[row - 1][col] + dp[row][col - 1];
        }
      }
    }

    dp[rows - 1][cols - 1]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
      2
    );

    assert_eq!(
      Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
      1
    );

    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);
  }
}
