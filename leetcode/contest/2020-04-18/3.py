from typing import List
from operator import itemgetter, attrgetter

class Solution:
  def getTriggerTime(self, increase: List[List[int]], requirements: List[List[int]]) -> List[int]:
    ctx = [0, 0, 0]
    result = [-1] * len(requirements)
    for i in range(len(requirements)):
      requirements[i].append(i)
    requirements.sort(key=itemgetter(0, 1, 2))

    if requirements[0][:3] == [0, 0, 0]:
      result[requirements[0][3]] = 0
      requirements.pop(0)

    for day in range(len(increase)):
      todayIncrease = increase[day]
      ctx[0] += todayIncrease[0]
      ctx[1] += todayIncrease[1]
      ctx[2] += todayIncrease[2]
      idx = 0
      while idx < len(requirements):
        requirement = requirements[idx]
        if ctx[0] < requirement[0]: break
        if ctx[0] >= requirement[0] and \
          ctx[1] >= requirement[1] and \
          ctx[2] >= requirement[2] and \
          result[requirement[3]] == -1:
          result[requirement[3]] = day + 1
          requirements.pop(idx)
        else:
          idx += 1
    return result

increase = [[2,8,4],[2,5,0],[10,9,8]]
requirements = [[2,11,3],[15,10,7],[9,17,12],[8,1,14]]

increase = [[0,4,5],[4,8,8],[8,6,1],[10,10,0]]
requirements = [[12,11,16],[20,2,6],[9,2,6],[10,18,3],[8,14,9]]

increase = [[1,1,1]]
requirements = [[0,0,0]]

s = Solution()
result = s.getTriggerTime(increase, requirements)
print(result)
