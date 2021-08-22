// dp[row][col] = triangle[row][col] + min(dp[row+1][col], dp[row+1][col+1])
impl Solution {
  pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    if triangle.is_empty() {
      return 0;
    }

    for row in (0..triangle.len() - 1).rev() {
      for col in 0..triangle[row].len() {
        triangle[row][col] =
          triangle[row][col] + triangle[row + 1][col].min(triangle[row + 1][col + 1])
      }
    }

    triangle[0][0]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
      11
    );
  }
}
