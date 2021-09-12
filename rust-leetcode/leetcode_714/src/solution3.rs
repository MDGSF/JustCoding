// dp[第i天][是否持有股票]
// dp[i][0] 从第0天到第i天，并且第i天没有持有股票，能够获得的最大收益
// dp[i][1] 从第0天到第i天，并且第i天持有股票，能够获得的最大收益
//
// dp[i][0] = max(dp[i-1][0], dp[i-1][1] + prices[i])
// dp[i][1] = max(dp[i-1][1], dp[i-1][0] - prices[i] - fee)
impl Solution {
  pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices
      .iter()
      .skip(1)
      .fold((0, -prices[0] - fee), |(dp_i_0, dp_i_1), price| {
        (
          std::cmp::max(dp_i_0, dp_i_1 + price),
          std::cmp::max(dp_i_1, dp_i_0 - price - fee),
        )
      })
      .0
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
  }
}
