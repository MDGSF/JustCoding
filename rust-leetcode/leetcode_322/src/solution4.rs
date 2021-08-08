impl Solution {
  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![std::i32::MAX; (amount + 1) as usize];
    dp[0] = 0;
    let len = dp.len();
    for &coin in coins.iter() {
      for i in coin as usize..len {
        if dp[i - coin as usize] != std::i32::MAX {
          dp[i] = dp[i].min(dp[i - coin as usize] + 1);
        }
      }
    }
    let last = *dp.last().unwrap();
    return if last == std::i32::MAX { -1 } else { last };
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
  }
}
