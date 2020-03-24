class Solution:
  def coinChange(self, coins: List[int], amount: int) -> int:
    dp = [0] + [float('inf') for _ in range(amount)]
    for coin in coins:
      for col in range(coin, amount + 1):
        dp[col] = min(dp[col], dp[col - coin] + 1)
    return dp[-1] if dp[-1] != float('inf') else -1
