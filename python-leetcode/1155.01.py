class Solution:
  # dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j - 2] + ... + dp[i - 1][j - f]
  def numRollsToTarget(self, d: int, f: int, target: int) -> int:
    m = {}
    def dfs(d, target):
      if d == 0:
        return 0 if target > 0 else 1
      if (d, target) in m:
        return m[(d, target)]
      result = 0
      for i in range(max(0, target-f), target):
        result += dfs(d-1, i)
      m[(d, target)] = result
      return result
    return dfs(d, target) % (10**9 + 7)

