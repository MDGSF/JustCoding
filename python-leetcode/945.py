from typing import List

class Solution:
  def minIncrementForUnique(self, A: List[int]) -> int:
    count = [0] * 80000
    for x in A:
      count[x] += 1

    result, taken = 0, 0
    for x in range(80000):
      if count[x] >= 2:
        taken += (count[x] - 1)
        result -= x * (count[x] - 1)
      elif taken > 0 and count[x] == 0:
        taken -= 1
        result += x
    return result

s = Solution()
result = s.minIncrementForUnique([1,2,2])
print(result)
