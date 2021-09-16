// dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1])
impl Solution {
  pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    for col in 1..cols {
      grid[0][col] += grid[0][col - 1];
    }
    for row in 1..rows {
      grid[row][0] += grid[row - 1][0];
    }
    for row in 1..rows {
      for col in 1..cols {
        grid[row][col] += grid[row - 1][col].min(grid[row][col - 1]);
      }
    }
    grid[rows - 1][cols - 1]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
      7
    );
  }
}
