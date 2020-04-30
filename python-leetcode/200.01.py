from typing import List

class Solution:
  def numIslands(self, grid: List[List[str]]) -> int:
    if len(grid) == 0: return 0
    rows, cols, count = len(grid), len(grid[0]), 0
    for row in range(rows):
      for col in range(cols):
        if grid[row][col] == '1':
          count += 1
          self.destoryIslandBFS(grid, row, col)
    return count

  def destoryIslandBFS(self, grid, row, col):
    rows, cols = len(grid), len(grid[0])
    queue = [(row, col)]
    while queue:
      curRow, curCol = queue.pop(0)
      for x, y in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        newRow, newCol = curRow + x, curCol + y
        if 0 <= newRow < rows and \
          0 <= newCol < cols and \
          grid[newRow][newCol] == '1':
          grid[newRow][newCol] = '0'
          queue.append((newRow, newCol))


grid = [["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]
s = Solution()
result = s.numIslands(grid)
print(result)
