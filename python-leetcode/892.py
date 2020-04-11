class Solution:
  def surfaceArea(self, grid: List[List[int]]) -> int:
    if len(grid) == 0: return 0
    surfaceArea, rows, cols = 0, len(grid), len(grid[0])
    for row in range(rows):
      for col in range(cols):
        if grid[row][col] > 0:
          surfaceArea += 2 # 上下两面
          for x, y in [(-1, 0), (1, 0), (0, -1), (0, 1)]: # 4个侧面
            newRow, newCol = row + x, col + y
            if newRow < 0 or newRow >= rows or newCol < 0 or newCol >= cols:
              surfaceArea += grid[row][col]
            elif grid[row][col] > grid[newRow][newCol]:
              surfaceArea += grid[row][col] - grid[newRow][newCol]
    return surfaceArea


