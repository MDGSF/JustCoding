class Solution:
  def numRollsToTarget(self, d: int, f: int, target: int) -> int:
    rows, cols = d + 1, target + 1
    dp = [[0 for _ in range(cols)] for _ in range(rows)]
    dp[0][0] = 1
    for row in range(1, rows):
      for col in range(row, cols):
        if row == col:
          dp[row][col] = 1
        else:
          start = col - f if col >= f else 0
          dp[row][col] = sum(dp[row-1][start:col]) % 1000000007
    return dp[-1][-1]

s = Solution()
result = s.numRollsToTarget(2, 6, 7)
print(result)
