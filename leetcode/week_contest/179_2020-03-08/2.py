from typing import List

class Solution:
  def numTimesAllBlue(self, light: List[int]) -> int:
    result, last, lightOn = 0, 0, [True] + [False]*len(light)
    for i, k in enumerate(light):
      lightOn[k] = True
      while last + 1 < len(lightOn) and lightOn[last + 1]:
        last += 1
      if last == i + 1:
        result += 1
    return result

light = [2,1,3,5,4] # 3
light = [3,2,4,1,5] # 2
light = [4,1,2,3] # 1
light = [2,1,4,3,6,5] # 3
light = [1,2,3,4,5,6] # 6
s = Solution()
result = s.numTimesAllBlue(light)
print(result)

