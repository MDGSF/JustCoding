// dp[第i天][是否持有股票]
// dp[i][0] 表示从第0天到第i天，并且第i天没有持有股票，最大收益
// dp[i][1] 表示从第0天到第i天，并且第i天持有股票，最大收益
//
// dp[i][0] = max(dp[i-1][0], dp[i-1][1] + prices[i])
// dp[i][1] = max(dp[i-1][1], dp[i-2][0] - prices[i])
impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
      return 0;
    }

    let n = prices.len();

    let mut dp = vec![vec![0; 2]; n];

    let i = 0;
    dp[i][0] = 0;
    dp[i][1] = -prices[i];

    let i = 1;
    dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
    dp[i][1] = std::cmp::max(dp[i - 1][1], -prices[i]);

    for i in 2..n {
      dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
      dp[i][1] = std::cmp::max(dp[i - 1][1], dp[i - 2][0] - prices[i]);
    }

    dp[n - 1][0]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
  }
}
