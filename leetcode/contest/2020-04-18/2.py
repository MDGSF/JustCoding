# 0
# 0->2 0->4
# 0->2->1  0->2->3  0->2->0

from typing import List
import copy

class Solution:
  def numWays(self, n: int, relation: List[List[int]], k: int) -> int:
    m = {}
    for oneRelation in relation:
      src, dst = oneRelation[0], oneRelation[1]
      if src in m:
        m[src].append(dst)
      else:
        m[src] = [dst]
    node = [[0]]
    for _ in range(k):
      nextNode = []
      for one in node:
        last = one[-1]
        if last in m:
          lastval = m[last]
          for v in lastval:
            newone = copy.deepcopy(one)
            newone.append(v)
            nextNode.append(newone)
      node = nextNode
    result = 0
    for one in node:
      if one[-1] == n - 1:
        result += 1
    return result

n = 5
relation = [[0,2],[2,1],[3,4],[2,3],[1,4],[2,0],[0,4]]
k = 3

n = 3
relation = [[0,2],[2,1]]
k = 2

s = Solution()
result = s.numWays(n, relation, k)
print(result)
