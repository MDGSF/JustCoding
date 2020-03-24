from typing import List

# 空间优化
class Solution:
  def massage(self, nums: List[int]) -> int:
    n = len(nums)
    if n == 0: return 0
    dp_i_0, dp_i_1 = 0, nums[0]
    for i in range(1, n):
      pre_i_0, pre_i_1 = dp_i_0, dp_i_1
      dp_i_0 = max(pre_i_0, pre_i_1)
      dp_i_1 = pre_i_0 + nums[i]
    return max(dp_i_0, dp_i_1)

nums = [1,2,3,1]
s = Solution()
result = s.massage(nums)
print(result)
