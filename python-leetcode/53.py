class Solution:
  # dp[i] = max(dp[i-1]+nums[i], nums[i])
  def maxSubArray(self, nums: List[int]) -> int:
    cur, result = nums[0], nums[0]
    for i in range(1, len(nums)):
      cur = max(cur + nums[i], nums[i])
      result = max(result, cur)
    return result
