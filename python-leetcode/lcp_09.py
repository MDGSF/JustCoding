from typing import List

class Solution:
  def minJump(self, jump: List[int]) -> int:
    dp = [0] * len(jump)
    for i in range(len(jump)-1, -1, -1):
      if i  + jump[i] >= len(jump):
        dp[i] = 1
      else:
        dp[i] = dp[i + jump[i]] + 1
      j = i + 1
      while j < len(jump) and j < i + jump[i] and dp[j] > dp[i]:
        dp[j] = min(dp[j], dp[i] + 1)
        j += 1
    return dp[0]

jump = [2, 5, 1, 1, 1, 1]
s = Solution()
result = s.minJump(jump)
print(result)
