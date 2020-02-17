from typing import List

class Solution:
  # dp[i][j] 表示由 coins[0:i] 组成 amount = j 的组合数
  # dp[rows][0] = 1
  # dp[0][1:cols] = 0
  # if j >= coins[i-1]
  #   dp[i][j] = dp[i-1][j] + dp[i][j - coins[i-1]]
  # else
  #   dp[i][j] = dp[i-1][j]
  def change(self, amount: int, coins: List[int]) -> int:
    dp = [1] + [0 for _ in range(amount)]
    for coin in coins:
      for col in range(coin, amount + 1):
        dp[col] += dp[col - coin]
    return dp[-1]

amount = 5
coins = [1,2,5]
s = Solution()
result = s.change(amount, coins)
print(result)
