from typing import List

class Solution:
  def gardenNoAdj(self, N: int, paths: List[List[int]]) -> List[int]:
    colors = {1, 2, 3, 4}
    visited = {}
    neighbor = {}
    for path in paths:
      src, dst = path[0], path[1]
      if src in neighbor:
        neighbor[src].add(dst)
      else:
        neighbor[src] = {dst}
      if dst in neighbor:
        neighbor[dst].add(src)
      else:
        neighbor[dst] = {src}
    for nodeId in range(1, N + 1):
      if nodeId in visited:
        continue
      curColors = colors.copy()
      if nodeId in neighbor:
        curNodeNeighbors = neighbor[nodeId]
        for curNeighbor in curNodeNeighbors:
          if curNeighbor in visited:
            curColors.discard(visited[curNeighbor])
      visited[nodeId] = curColors.pop()
    return [visited[i] for i in range(1, N + 1)]


N = 3
paths = [[1,2],[2,3],[3,1]]
s = Solution()
result = s.gardenNoAdj(N, paths)
print(result)
