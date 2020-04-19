from typing import List

# 对每个 1 做 bfs 搜索
class Solution:

  def updateMatrix(self, matrix: List[List[int]]) -> List[List[int]]:
    rows, cols = len(matrix), len(matrix[0])
    result = [[0 for _ in range(cols)] for _ in range(rows)]
    for row in range(rows):
      for col in range(cols):
        if matrix[row][col] > 0:
          self.findZero(matrix, result, row, col)
    return result

  def findZero(self, matrix, result, row, col):
    rows, cols = len(matrix), len(matrix[0])
    count = 1
    queue = [(row, col)]
    while queue:
      curLevelSize = len(queue)
      for _ in range(curLevelSize):
        curRow, curCol = queue.pop(0)
        for x, y in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
          newRow, newCol = curRow + x, curCol + y
          if 0 <= newRow < rows and 0 <= newCol < cols:
            if matrix[newRow][newCol] == 0:
              result[row][col] = count
              return
            else:
              queue.append((newRow, newCol))
      count += 1


matrix = [
  [0, 0, 0],
  [0, 1, 0],
  [1, 1, 1],
]

s = Solution()
result = s.updateMatrix(matrix)
print(result)
