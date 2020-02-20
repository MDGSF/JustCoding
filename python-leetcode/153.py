from typing import List

class Solution:
  def findMin(self, nums: List[int]) -> int:
    left, right = 0, len(nums) - 1
    while left < right:
      mid = left + (right - left) // 2
      if nums[mid] > nums[right]:
        left = mid + 1
      else:
        right = mid
    return nums[left]

nums = [3,4,5,1,2]
s = Solution()
result = s.findMin(nums)
print(result)
