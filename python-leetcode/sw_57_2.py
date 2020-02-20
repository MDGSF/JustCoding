from typing import List

class Solution:
  def findContinuousSequence(self, target: int) -> List[List[int]]:
    left, right, middle = 1, 2, (1 + target) // 2
    result = []
    s = left + right
    while left < middle:
      if s == target:
        result.append([i for i in range(left, right + 1)])
        right += 1
        s += right
      elif s < target:
        right += 1
        s += right
      else:
        s -= left
        left += 1
    return result

s = Solution()
result = s.findContinuousSequence(9)
print(result)
