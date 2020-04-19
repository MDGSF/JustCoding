from typing import List

class Solution:
  def minSubsequence(self, nums: List[int]) -> List[int]:
    s = sum(nums)
    nums.sort(reverse = True)
    cur = 0
    result = []
    for num in nums:
      cur += num
      result.append(num)
      if cur > s - cur:
        return result

nums = [4,3,10,9,8]
nums = [4,4,7,6,7]
nums = [6]
s = Solution()
result = s.minSubsequence(nums)
print(result)
