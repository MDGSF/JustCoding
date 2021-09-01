// dp[i] 表示总金额为i时，硬币的组合数
impl Solution {
  pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let mut dp = vec![0; (amount + 1) as usize];
    dp[0] = 1;
    let len = dp.len();
    for &coin in coins.iter() {
      for i in coin as usize..len {
        dp[i] += dp[i - coin as usize]
      }
    }
    dp[len - 1]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    assert_eq!(Solution::change(3, vec![2]), 0);
    assert_eq!(Solution::change(10, vec![10]), 1);
  }
}
