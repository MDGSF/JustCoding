from typing import List

class Solution:
  def maxValue(self, grid: List[List[int]]) -> int:
    rows, cols = len(grid), len(grid[0])
    for row in range(rows):
      for col in range(cols):
        if row == 0 and col == 0: continue
        if row == 0: grid[row][col] += grid[row][col - 1]
        elif col == 0: grid[row][col] += grid[row - 1][col]
        else: grid[row][col] += max(grid[row - 1][col], grid[row][col - 1])
    return grid[-1][-1]

grid = [
  [1,3,1],
  [1,5,1],
  [4,2,1]
]

s = Solution()
result = s.maxValue(grid)
print(result)

