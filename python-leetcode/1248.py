from typing import List

class Solution:
  def numberOfSubarrays(self, nums: List[int], k: int) -> int:
    i = count = result = 0
    for j in range(len(nums)):
      if nums[j] & 1:
        k -= 1
        count = 0
      while k == 0:
        k += nums[i] & 1
        i += 1
        count += 1
      result += count
    return result

nums = [1,1,2,1,1]
k = 3

nums = [2,2,2,1,2,2,1,2,2,2]
k = 2

s = Solution()
result = s.numberOfSubarrays(nums, k)
print(result)
