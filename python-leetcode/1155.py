class Solution:
  def numRollsToTarget(self, d: int, f: int, target: int) -> int:
    if target > f * d or target < d: return 0
    dp = [1] * f
    for row in range(2, d + 1):
      limit = min(target, f * d)
      dp = [sum(dp[max(0, col - f):col]) % 1000000007 for col in range(limit)]
    return dp[-1]

s = Solution()
result = s.numRollsToTarget(30, 30, 500)
print(result)
