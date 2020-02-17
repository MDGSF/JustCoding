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
    rows, cols = len(coins) + 1, amount + 1
    dp = [[0 for _ in range(cols)] for _ in range(rows)]
    for row in range(rows):
      dp[row][0] = 1
    for col in range(1, cols):
      dp[0][col] = 0
    for row in range(1, rows):
      for col in range(1, cols):
        if col >= coins[row-1]:
          dp[row][col] = dp[row-1][col] + dp[row][col - coins[row - 1]]
        else:
          dp[row][col] = dp[row-1][col]
    return dp[-1][-1]

amount = 5
coins = [1,2,5]
s = Solution()
result = s.change(amount, coins)
print(result)
