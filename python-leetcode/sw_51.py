from typing import List

class Solution:
  def reversePairs(self, nums: List[int]) -> int:
    return self.reversePairsCore(nums, 0, len(nums) - 1)

  def reversePairsCore(self, nums, start, end):
    if start >= end:
      return 0
    length = (end - start) // 2
    left = self.reversePairsCore(nums, start, start + length)
    right = self.reversePairsCore(nums, start + length + 1, end)
    i, j, count = start + length, end, 0
    while i >= start and j >= start + length + 1:
      if nums[i] > nums[j]:
        count += j - start - length
        i -= 1
      else:
        j -= 1
    nums[start:end+1] = sorted(nums[start:end+1])
    return left + right + count

nums = [7, 5, 6, 4]
s = Solution()
result = s.reversePairs(nums)
print(result)
