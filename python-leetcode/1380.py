class Solution:
  def luckyNumbers (self, matrix: List[List[int]]) -> List[int]:
    result, rows, cols = [], len(matrix), len(matrix[0])
    for row in range(rows):
      minNum, minCol = matrix[row][0], 0
      for col in range(1, cols):
        if matrix[row][col] < minNum:
          minNum, minCol = matrix[row][col], col
      success = True
      for i in range(rows):
        if matrix[i][minCol] > minNum:
          success = False
          break
      if success:
        result.append(minNum)
    return result
