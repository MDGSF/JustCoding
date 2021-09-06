impl Solution {
  pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut dp = vec![vec![0; cols + 1]; rows + 1];
    let mut result = 0;
    for row in 1..=rows {
      for col in 1..=cols {
        if matrix[row - 1][col - 1] == '1' {
          dp[row][col] = dp[row - 1][col]
            .min(dp[row][col - 1])
            .min(dp[row - 1][col - 1])
            + 1;
          result = result.max(dp[row][col]);
        }
      }
    }
    result * result
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::maximal_square(vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
      ],),
      4
    );
  }
}
