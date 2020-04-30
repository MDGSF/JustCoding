from typing import List

class Solution:
  def numberOfSubarrays(self, nums: List[int], k: int) -> int:
    result, oddSum, prefix = 0, 0, [1] + [0] * len(nums)
    for num in nums:
      if num % 2 == 1:
        oddSum += 1
      prefix[oddSum] += 1
      if oddSum >= k:
        result += prefix[oddSum - k]
    return result

nums = [1,1,2,1,1]
k = 3
s = Solution()
result = s.numberOfSubarrays(nums, k)
print(result)
