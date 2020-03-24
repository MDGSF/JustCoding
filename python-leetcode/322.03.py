from typing import List

class Solution:
  def coinChange(self, coins: List[int], amount: int) -> int:
    m = {}
    def dp(n):
      if n == 0: return 0
      if n < 0: return -1
      if n in m: return m[n]
      result = float('inf')
      for coin in coins:
        sub = dp(n - coin)
        if sub == -1: continue
        result = min(result, sub + 1)
      m[n] = result if result != float('inf') else -1
      return m[n]
    return dp(amount)

coins = [1, 2, 5]
amount = 11
s = Solution()
result = s.coinChange(coins, amount)
print(result)
