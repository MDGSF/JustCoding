class Solution:
  def maxProfit(self, prices: List[int]) -> int:
    dp_i_0, dp_i_1 = 0, float('-inf')
    for i in range(len(prices)):
      pre_i_0, pre_i_1 = dp_i_0, dp_i_1
      dp_i_0 = max(pre_i_0, pre_i_1 + prices[i])
      dp_i_1 = max(pre_i_1, pre_i_0 - prices[i])
    return dp_i_0

