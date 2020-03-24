from typing import List

# dp[i][0] 表示从 0 到第 i 个预约，不接第 i 个预约，总预约时间最长分钟数
# dp[i][1] 表示从 0 到第 i 个预约，接受第 i 个预约，总预约时间最长分钟数
# dp[i][0] = max(dp[i-1][0], dp[i-1][1])
# dp[i][1] = dp[i-1][0] + nums[i]
# result = max(dp[-1][0], dp[-1][1])
class Solution:
  def massage(self, nums: List[int]) -> int:
    n = len(nums)
    if n == 0: return 0
    dp = [[0 for _ in range(2)] for _ in range(n)]
    dp[0][0] = 0
    dp[0][1] = nums[0]
    for i in range(1, n):
      dp[i][0] = max(dp[i-1][0], dp[i-1][1])
      dp[i][1] = dp[i-1][0] + nums[i]
    return max(dp[-1][0], dp[-1][1])

nums = [1,2,3,1]
s = Solution()
result = s.massage(nums)
print(result)
