from typing import List
import collections

# 给所有的 0 增加一个超级源
class Solution:

  def updateMatrix(self, matrix: List[List[int]]) -> List[List[int]]:
    rows, cols = len(matrix), len(matrix[0])
    count = 1
    queue = collections.deque()
    visited = set()
    for row in range(rows):
      for col in range(cols):
        if matrix[row][col] == 0:
          queue.append((row, col))
          visited.add((row, col))
    while queue:
      curLevelSize = len(queue)
      for _ in range(curLevelSize):
        curRow, curCol = queue.popleft()
        for x, y in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
          newRow, newCol = curRow + x, curCol + y
          if 0 <= newRow < rows and 0 <= newCol < cols \
            and matrix[newRow][newCol] > 0 \
            and (newRow, newCol) not in visited:
              matrix[newRow][newCol] = count
              queue.append((newRow, newCol))
              visited.add((newRow, newCol))
      count += 1
    return matrix


matrix = [
  [0, 0, 0],
  [0, 1, 0],
  [1, 1, 1],
]

s = Solution()
result = s.updateMatrix(matrix)
print(result)
