impl Solution {
  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![std::i32::MAX; (amount + 1) as usize];
    dp[0] = 0;
    let len = dp.len();
    for i in 0..len {
      let i = i as i32;
      for &coin in coins.iter() {
        if i < coin {
          continue;
        }
        if dp[(i - coin) as usize] != std::i32::MAX {
          dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
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
