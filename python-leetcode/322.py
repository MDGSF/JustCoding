class Solution:
  def coinChange(self, coins: List[int], amount: int) -> int:
    dp = [0] + [float('inf') for _ in range(amount)]
    for i in range(len(dp)):
      for coin in coins:
        if i - coin < 0: continue
        dp[i] = min(dp[i], dp[i - coin] + 1)
    return dp[-1] if dp[-1] != float('inf') else -1

