class Solution:
  def maxProfit(self, prices: List[int]) -> int:
    return self.maxProfit_k_any(2, prices)

  def maxProfit_k_any(self, max_k: int, prices: List[int]) -> int:
    n = len(prices)
    if max_k > n / 2:
      return self.maxProfit_k_inf(prices)
    dp = [[[0 for _ in range(2)] for _ in range(max_k + 1)] for _ in range(n)]
    for i in range(n):
      for k in range(max_k, 0, -1):
        if i - 1 == -1:
          dp[i][k][0] = 0
          dp[i][k][1] = -prices[i]
          continue
        dp[i][k][0] = max(dp[i-1][k][0], dp[i-1][k][1] + prices[i])
        dp[i][k][1] = max(dp[i-1][k][1], dp[i-1][k-1][0] - prices[i])
    return dp[n-1][max_k][0]

  def maxProfit_k_inf(self, prices: List[int]) -> int:
    dp_i_0, dp_i_1 = 0, float('-inf')
    for i in range(len(prices)):
      pre_i_0, pre_i_1 = dp_i_0, dp_i_1
      dp_i_0 = max(pre_i_0, pre_i_1 + prices[i])
      dp_i_1 = max(pre_i_1, pre_i_0 - prices[i])
    return dp_i_0

