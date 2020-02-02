class Solution:
  def minDistance(self, word1: str, word2: str) -> int:
    rows = len(word1) + 1
    cols = len(word2) + 1
    dp = [[0 for _ in range(cols)] for _ in range(rows)]
    for row in range(1, rows):
      dp[row][0] = dp[row-1][0] + 1
    for col in range(1, cols):
      dp[0][col] = col
    for row in range(1, rows):
      for col in range(1, cols):
        if word1[row-1] == word2[col-1]:
          dp[row][col] = 1 + min(dp[row-1][col], dp[row][col-1],
              dp[row-1][col-1] - 1)
        else:
          dp[row][col] = 1 + min(dp[row-1][col], dp[row][col-1],
              dp[row-1][col-1])
    return dp[-1][-1]

s = Solution()
result = s.minDistance("horse", "ros")
print(result)
