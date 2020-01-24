from typing import List
class Solution:

  def threeSum(self, nums: List[int]) -> List[List[int]]:
    result = []
    nums.sort()
    n = len(nums)
    for i in range(n):
      if i > 0 and nums[i] == nums[i - 1]:
        continue
      target = -nums[i]
      start, end = i + 1, n - 1
      while start < end:
        s = nums[start] + nums[end]
        if s > target:
          end -= 1
        elif s < target:
          start += 1
        else:
          result.append([nums[i], nums[start], nums[end]])

          while start + 1 < n and nums[start] == nums[start + 1]:
            start += 1
          start += 1

          while end - 1 >= 0 and nums[end] == nums[end - 1]:
            end -= 1
          end -= 1
    return result


s = Solution()
result = s.threeSum([-1, 0, 1, 2, -1, -4])
print(result)
