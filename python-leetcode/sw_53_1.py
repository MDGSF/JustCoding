from typing import List

class Solution:
  def search(self, nums: List[int], target: int) -> int:
    def findleft():
      left, right = 0, len(nums) - 1
      while left <= right:
        mid = (left + right) // 2
        if nums[mid] > target:
          right = mid - 1
        elif nums[mid] < target:
          left = mid + 1
        else:
          if mid == 0 or nums[mid - 1] != target:
            return mid
          else:
            right = mid - 1
      return -1
    def findright():
      left, right = 0, len(nums) - 1
      while left <= right:
        mid = (left + right) // 2
        if nums[mid] > target:
          right = mid - 1
        elif nums[mid] < target:
          left = mid + 1
        else:
          if mid == len(nums) - 1 or nums[mid + 1] != target:
            return mid
          else:
            left = mid + 1
      return -1
    right = findright()
    if right == -1:
      return 0
    return right - findleft() + 1

nums = [5,7,7,8,8,10]
target = 8
s = Solution()
result = s.search(nums, target)
print(result)
