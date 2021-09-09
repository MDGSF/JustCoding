// dp[第i天][买入股票的次数][当前是否持有股票]
// dp表示从 0 到第 i 天的最大利益
//
// dp[i][k][0] 表示第i天，买入股票k次，当前没有持有股票
// dp[i][k][1] 表示第i天，买入股票k次，当前持有股票
//
// dp[i][k][0] = max(dp[i-1][k][0], dp[i-1][k][1] + prices[i])
// dp[i-1][k][0] 表示在第i-1天，买入股票k次，当前没有持有股票
// dp[i-1][k][1] + prices[i] 表示在第i-1天，买入股票k次，并持有股票，
//                               在第i天卖出股票
//
// dp[i][k][1] = max(dp[i-1][k][1], dp[i-1][k-1][0] - prices[i])
// dp[i-1][k][1] 表示在第i-1天，买入股票k次，当前持有股票
// dp[i-1][k-1][0] - prices[i] 表示在第i-1天，买入股票k-1次，没有持有股票
//                                 在第i天买入股票
impl Solution {
  pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let max_k = k as usize;
    let n = prices.len();

    if max_k == 0 || n == 0 {
      return 0;
    }

    let mut dp = vec![vec![vec![0; 2]; max_k + 1]; n];
    for i in 0..n {
      for k in (1..=max_k).rev() {
        if i == 0 {
          dp[i][k][0] = 0;
          dp[i][k][1] = -prices[i];
        } else {
          dp[i][k][0] = std::cmp::max(dp[i - 1][k][0], dp[i - 1][k][1] + prices[i]);
          dp[i][k][1] = std::cmp::max(dp[i - 1][k][1], dp[i - 1][k - 1][0] - prices[i]);
        }
      }
    }
    dp[n - 1][max_k][0]
  }
}

pub struct Solution;
