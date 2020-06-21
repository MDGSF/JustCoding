from typing import List
import copy


class Solution:

  def avoidFlood(self, rains: List[int]) -> List[int]:

    def innerAvoidFlood(rainsStartIdx, s, result):
      if rainsStartIdx >= len(rains):
        return True, result
      lakeID = rains[rainsStartIdx]
      if lakeID > 0:
        if lakeID in s:
          return False, []
        return innerAvoidFlood(rainsStartIdx + 1, s | {lakeID}, result + [-1])
      elif lakeID == 0:
        if len(s) == 0:
          return innerAvoidFlood(rainsStartIdx + 1, s, result + [1])
        else:
          for waterLakeID in s:
            news = copy.deepcopy(s)
            news.remove(waterLakeID)
            subret1, subret2 = innerAvoidFlood(rainsStartIdx + 1, news,
                                               result + [waterLakeID])
            if subret1 == False:
              continue
            else:
              return True, subret2
          return False, []
      return False, []

    ret1, ret2 = innerAvoidFlood(0, set(), [])
    return ret2


s = Solution()

# 输入：rains = [1,2,3,4]
# 输出：[-1,-1,-1,-1]
# rains = [1,2,3,4]
# result = s.avoidFlood(rains)
# print(result)

# 输入：rains = [1,2,0,0,2,1]
# 输出：[-1,-1,2,1,-1,-1]
# rains = [1,2,0,0,2,1]
# result = s.avoidFlood(rains)
# print(result)

# rains = [1,2,3,4]
# rains = [1,2,0,0,2,1]
# rains = [1,2,0,1,2]
# rains = [69,0,0,0,69]
# rains = [10,20,20]
# rains = [1,2,0,2,3,0,1]
rains = [3, 5, 4, 0, 1, 0, 1, 5, 2, 8, 9]  # [-1,-1,-1,5,-1,1,-1,-1,-1,-1,-1]
result = s.avoidFlood(rains)
print(result)
