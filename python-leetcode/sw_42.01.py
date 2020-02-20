class Solution:
  # dp[i] 表示从 0 到 i，并以 i 结尾的子数组最大和
  # dp[i] = max(dp[i-1] + nums[i], nums[i])
  # result = max(dp[0], d[1], ..., dp[i])
  def maxSubArray(self, nums: List[int]) -> int:
    dp = [float('-inf') for _ in range(len(nums))]
    dp[0] = nums[0]
    for i in range(1, len(nums)):
      dp[i] = max(dp[i-1]+nums[i], nums[i])
    return max(dp)
