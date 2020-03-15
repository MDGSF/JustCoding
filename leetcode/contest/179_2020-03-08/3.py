from typing import List

class Solution:
  def numOfMinutes(self, n: int, headID: int, manager: List[int], informTime: List[int]) -> int:
    dp = [0] * n
    queue = [k for k, v in enumerate(informTime) if v == 0]
    while len(queue) > 0:
      parent = set()
      for i in range(len(queue)):
        node = queue[i]
        parentNode = manager[node]
        if parentNode != -1:
          parent.add(parentNode)
          dp[parentNode] = max(dp[parentNode],
              informTime[parentNode] + dp[node])
      queue = list(parent)
    return max(dp)


n = 1
headID = 0
manager = [-1]
informTime = [0]
# 0

# n = 6
# headID = 2
# manager = [2,2,-1,2,2,2]
# informTime = [0,0,1,0,0,0]
# 1


# n = 7
# headID = 6
# manager = [1,2,3,4,5,6,-1]
# informTime = [0,6,5,4,3,2,1]
# 21

# n = 15
# headID = 0
# manager = [-1,0,0,1,1,2,2,3,3,4,4,5,5,6,6]
# informTime = [1,1,1,1,1,1,1,0,0,0,0,0,0,0,0]
# 3

# n = 4
# headID = 2
# manager = [3,3,-1,2]
# informTime = [0,0,162,914]
# 1076

s = Solution()
result = s.numOfMinutes(n, headID, manager, informTime)
print(result)

