from typing import List

class Solution:
  def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
    self.maxArea, rows, cols = 0, len(grid), len(grid[0])
    def bfs(row, col):
      grid[row][col] = 0
      curArea = 1
      queue = [(row, col)]
      while queue:
        node = queue.pop(0)
        curRow, curCol = node[0], node[1]
        for x, y in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
          newRow, newCol = curRow + x, curCol + y
          if 0 <= newRow < rows and 0 <= newCol < cols and grid[newRow][newCol] == 1:
            grid[newRow][newCol] = 0
            curArea += 1
            queue.append((newRow, newCol))
      self.maxArea = max(self.maxArea, curArea)
    for row in range(rows):
      for col in range(cols):
        if grid[row][col] == 1:
          bfs(row, col)
    return self.maxArea

grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],
 [0,0,0,0,0,0,0,1,1,1,0,0,0],
 [0,1,1,0,1,0,0,0,0,0,0,0,0],
 [0,1,0,0,1,1,0,0,1,0,1,0,0],
 [0,1,0,0,1,1,0,0,1,1,1,0,0],
 [0,0,0,0,0,0,0,0,0,0,1,0,0],
 [0,0,0,0,0,0,0,1,1,1,0,0,0],
 [0,0,0,0,0,0,0,1,1,0,0,0,0]]

s = Solution()
result = s.maxAreaOfIsland(grid)
print(result)
