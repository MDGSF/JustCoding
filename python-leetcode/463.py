from typing import List

class Solution:
  def islandPerimeter(self, grid: List[List[int]]) -> int:
    self.rows, self.cols = len(grid), len(grid[0])
    self.result = 0
    visited = set()
    for row in range(self.rows):
      for col in range(self.cols):
        if grid[row][col] == 1 and (row, col) not in visited:
          self.recursion(grid, row, col, visited)
    return self.result

  def recursion(self, grid, row, col, visited):
    s = set()
    s.add((row, col))
    while len(s) > 0:
      node = s.pop()
      visited.add(node)
      for x, y in [[1, 0], [-1, 0], [0, 1], [0, -1]]:
        newRow, newCol = node[0] + x, node[1] + y
        newNodeIsValid = self.isValid(newRow, newCol)
        if (not newNodeIsValid) or \
          (newNodeIsValid and grid[newRow][newCol] == 0) :
          self.result += 1
        if newNodeIsValid and \
          grid[newRow][newCol] != 0 and \
          (newRow, newCol) not in visited:
          s.add((newRow, newCol))

  def isValid(self, row, col):
    if 0 <= row < self.rows and 0 <= col < self.cols:
      return True
    return False

grid = [[1,1],[1,1]]
s = Solution()
result = s.islandPerimeter(grid)
print(result)
