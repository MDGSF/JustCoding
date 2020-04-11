from typing import List

class Solution:
  def maxSatisfaction(self, satisfaction: List[int]) -> int:
    satisfaction.sort()
    maxval = 0
    curtime = 1
    for i in range(len(satisfaction)):
      maxval += satisfaction[i] * curtime
      curtime += 1

    curval = maxval
    for i in range(len(satisfaction)):
      for j in range(i, len(satisfaction)):
        curval -= satisfaction[j]
      if curval > maxval:
        maxval = curval
    return maxval

satisfaction = [-1,-8,0,5,-9]
satisfaction = [4,3,2]
satisfaction = [-1,-4,-5]
satisfaction = [-2,5,-1,0,3,-3]
s = Solution()
result = s.maxSatisfaction(satisfaction)
print(result)
