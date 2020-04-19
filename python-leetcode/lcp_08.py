from typing import List

class Solution:
  def getTriggerTime(self, increase: List[List[int]], requirements: List[List[int]]) -> List[int]:
    n, r = len(increase), len(requirements)
    preSum = [[0, 0, 0] for _ in range(n + 1)]
    result = [-1] * r
    for i in range(n):
      preSum[i + 1][0] = preSum[i][0] + increase[i][0]
      preSum[i + 1][1] = preSum[i][1] + increase[i][1]
      preSum[i + 1][2] = preSum[i][2] + increase[i][2]
    for i in range(r):
      left, right = 0, n
      while left <= right:
        mid = left + (right - left) // 2
        if preSum[mid][0] >= requirements[i][0] and \
          preSum[mid][1] >= requirements[i][1] and \
          preSum[mid][2] >= requirements[i][2]:
          result[i] = mid
          right = mid - 1
        else:
          left = mid + 1
    return result


increase = [[2,8,4],[2,5,0],[10,9,8]]
requirements = [[2,11,3],[15,10,7],[9,17,12],[8,1,14]]

s = Solution()
result = s.getTriggerTime(increase, requirements)
print(result)
