from typing import List

class Solution:
  def singleNumber(self, nums: List[int]) -> int:
    nums.sort()
    for i in range(1, len(nums) - 1):
      if nums[i] != nums[i-1] and nums[i] != nums[i+1]:
        return nums[i]
    return nums[0] if nums[0] != nums[1] else nums[-1]


nums = [3, 4, 3, 3]

s = Solution()
result = s.singleNumber(nums)
print(result)

