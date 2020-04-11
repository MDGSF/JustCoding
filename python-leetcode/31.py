from typing import List

class Solution:
  def nextPermutation(self, nums: List[int]) -> None:
    """
    Do not return anything, modify nums in-place instead.
    """
    n = len(nums)
    idx = -1
    for i in range(n - 2, -1, -1):
      if nums[i] < nums[i + 1]:
        idx = i
        break
    if idx == -1:
      nums.sort()
      return
    nums[(idx+1):] = sorted(nums[(idx+1):])
    for i in range(idx + 1, n):
      if nums[idx] < nums[i]:
        nums[idx], nums[i] = nums[i], nums[idx]
        break

nums = [1, 2, 3]
s = Solution()
s.nextPermutation(nums)
print(nums)
