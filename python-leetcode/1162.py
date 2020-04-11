from typing import List
import collections


class Solution:

  def maxDistance(self, grid: List[List[int]]) -> int:
    rows, cols = len(grid), len(grid[0])
    result = -1
    visited = set()
    queue = set()

    def isValid(row, col):
      return 0 <= row < rows and 0 <= col < cols

    for row in range(rows):
      for col in range(cols):
        if grid[row][col] == 1:
          queue.add((row, col))

    if len(queue) == rows * cols or len(queue) == 0:
      return -1

    while queue:
      newLevel = set()
      result += 1
      for _ in range(len(queue)):
        node = queue.pop()
        for x, y in [[1, 0], [-1, 0], [0, 1], [0, -1]]:
          newRow, newCol = node[0] + x, node[1] + y
          if isValid(newRow, newCol) and \
           (newRow, newCol) not in visited and \
            grid[newRow][newCol] == 0:
            newLevel.add((newRow, newCol))
            visited.add((newRow, newCol))
            grid[newRow][newCol] = 2
      queue = newLevel
    return result

grid = [[1,0,0,0,0,1,0,0,0,1],[1,1,0,1,1,1,0,1,1,0],[0,1,1,0,1,0,0,1,0,0],[1,0,1,0,1,0,0,0,0,0],[0,1,0,0,0,1,1,0,1,1],[0,0,1,0,0,1,0,1,0,1],[0,0,0,1,1,1,1,0,0,1],[0,1,0,0,1,0,0,1,0,0],[0,0,0,0,0,1,1,1,0,0],[1,1,0,1,1,1,1,1,0,0]]
# 2

# grid = [[1,0,1],[0,0,0],[1,0,1]] # 2

grid = [[1,0,0],[0,0,0],[0,0,0]] # 4

s = Solution()
result = s.maxDistance(grid)
print(result)

