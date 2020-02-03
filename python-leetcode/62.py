class Solution:
  def uniquePaths(self, m: int, n: int) -> int:
    rows, cols = n, m
    dp = [[0 for _ in range(cols)] for _ in range(rows)]
    dp[rows-1][cols-1] = 1
    for col in range(cols-2, -1, -1):
      dp[rows-1][col] = 1
    for row in range(rows-2, -1, -1):
      dp[row][cols-1] = 1
    for row in range(rows-2, -1, -1):
      for col in range(cols-2, -1, -1):
        dp[row][col] = dp[row+1][col] + dp[row][col+1]
    return dp[0][0]

