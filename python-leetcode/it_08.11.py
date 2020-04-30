
# 和 518 题相同

class Solution:
  def waysToChange(self, n: int) -> int:
    dp = [1] + [0] * n
    coins = [25, 10, 5, 1]
    for coin in coins:
      for col in range(coin, n + 1):
        dp[col] += dp[col - coin]
    return dp[-1] % (10**9 + 7)
