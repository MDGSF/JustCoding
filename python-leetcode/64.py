class Solution:
  def minPathSum(self, grid: List[List[int]]) -> int:
    rows = len(grid)
    cols = len(grid[0])
    for row in range(rows-2, -1, -1):
      grid[row][cols-1] += grid[row+1][cols-1]
    for col in range(cols-2, -1, -1):
      grid[rows-1][col] += grid[rows-1][col+1]
    for row in range(rows-2, -1, -1):
      for col in range(cols-2, -1, -1):
        grid[row][col] += min(grid[row+1][col], grid[row][col+1])
    return grid[0][0]

