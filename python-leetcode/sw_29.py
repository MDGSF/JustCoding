class Solution:
  def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
    if len(matrix) == 0: return []
    rows, cols, result, start = len(matrix), len(matrix[0]), [], 0
    while start * 2 < rows and start * 2 < cols:
      self.printMatrixInCircle(matrix, result, start)
      start += 1
    return result

  # start = 0 第一圈
  # start = 1 第二圈
  # ...
  def printMatrixInCircle(self, matrix, result, start):
    rows, cols = len(matrix), len(matrix[0])
    endRow, endCol = rows - 1 - start, cols - 1 - start

    # left -> right
    for col in range(start, endCol + 1):
      result.append(matrix[start][col])

    # top -> down
    for row in range(start + 1, endRow + 1):
      result.append(matrix[row][endCol])

    # right -> left
    if start < endRow and start < endCol:
      for col in range(endCol - 1, start - 1, -1):
        result.append(matrix[endRow][col])

    # down -> top
    if start < endRow - 1 and start < endCol:
      for row in range(endRow - 1, start, -1):
        result.append(matrix[row][start])


