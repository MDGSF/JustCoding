import collections

class Solution:
  def orangesRotting(self, grid: List[List[int]]) -> int:
    rows, cols = len(grid), len(grid[0])
    freshNumber = 0
    timeUsed = 0
    queue = collections.deque()
    for row in range(rows):
      for col in range(cols):
        if grid[row][col] == 2:
          queue.append((row, col, 0))
        elif grid[row][col] == 1:
          freshNumber += 1
    while queue:
      row, col, timeUsed = queue.popleft()
      for dirs in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
        newRow, newCol = row + dirs[0], col + dirs[1]
        if 0 <= newRow < rows and 0 <= newCol < cols and grid[newRow][newCol] == 1:
          freshNumber -= 1
          grid[newRow][newCol] = 2
          queue.append((newRow, newCol, timeUsed + 1))
    return -1 if freshNumber > 0 else timeUsed
