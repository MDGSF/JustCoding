// dp[i] 表示正整数i的结果
// dp[i] = min(dp[i - 完全平方数] + 1)
// result = dp[n]
impl Solution {
  pub fn num_squares(n: i32) -> i32 {
    let len: usize = (n + 1) as usize;
    let mut dp = vec![std::i32::MAX; len];
    dp[0] = 0;
    for i in 1..len {
      let mut j = 1;
      while i >= j * j {
        dp[i] = dp[i].min(dp[i - j * j] + 1);
        j += 1;
      }
    }
    dp[n as usize]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
  }
}
