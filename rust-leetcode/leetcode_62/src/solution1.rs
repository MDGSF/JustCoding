impl Solution {
  pub fn unique_paths(m: i32, n: i32) -> i32 {
    let rows: usize = m as usize;
    let cols: usize = n as usize;

    let mut dp = vec![vec![0; cols]; rows];

    for row in 0..rows {
      dp[row][0] = 1;
    }

    for col in 0..cols {
      dp[0][col] = 1;
    }

    for row in 1..rows {
      for col in 1..cols {
        dp[row][col] = dp[row - 1][col] + dp[row][col - 1];
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
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(Solution::unique_paths(3, 2), 3);
    assert_eq!(Solution::unique_paths(7, 3), 28);
    assert_eq!(Solution::unique_paths(3, 3), 6);
  }
}
