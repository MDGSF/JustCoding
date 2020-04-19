# 5355. T 秒后青蛙的位置

from typing import List

class Solution:
  def frogPosition(self, n: int, edges: List[List[int]], t: int, target: int) -> float:
    m = {}
    for edge in edges:
      fromNode, toNode = edge[0], edge[1]
      if fromNode not in m:
        m[fromNode] = set()
      m[fromNode].add(toNode)
      if toNode not in m:
        m[toNode] = set()
      m[toNode].add(fromNode)
    queue = [(1, 1)]
    visited = set()
    while len(queue) > 0 and t >= 0:
      nextLevel = []
      curLevelSize = len(queue)
      for i in range(curLevelSize):
        node, result = queue[i]
        if node == target:
          return result
        if node not in m:
          continue
        if node in visited:
          continue
        visited.add(node)
        children = m[node]
        result = result * (1 / len(children))
        for child in children:
          if child not in visited:
            nextLevel.append((child, result))
      queue = nextLevel
      t -= 1
    return 0

n = 7
edges = [[1,2],[1,3],[1,7],[2,4],[2,6],[3,5]]
t = 2
target = 4
# 0.16666666666666666

s = Solution()
result = s.frogPosition(n, edges, t, target)
print(result)

