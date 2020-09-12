import functools
from typing import List

class Solution:
  def minNumber(self, nums: List[int]) -> str:
    def cmp(x, y):
      a, b = x + y, y + x
      if a > b: return 1
      elif a < b: return -1
      else: return 0
    strs = [str(num) for num in nums]
    strs.sort(key = functools.cmp_to_key(cmp))
    return ''.join(strs)

#nums = [10, 2]
nums = [3,30,34,5,9]

s = Solution()
result = s.minNumber(nums)
print(result)

