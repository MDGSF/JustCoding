from typing import List

class Solution:
  def coinChange(self, coins: List[int], amount: int) -> int:
    def dp(n):
      if n == 0: return 0
      if n < 0: return -1
      result = float('inf')
      for coin in coins:
        sub = dp(n - coin)
        if sub == -1: continue
        result = min(result, sub + 1)
      return result if result != float('inf') else -1
    return dp(amount)

coins = [1, 2, 5]
amount = 11
s = Solution()
result = s.coinChange(coins, amount)
print(result)
