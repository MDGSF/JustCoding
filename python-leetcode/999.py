from typing import List

class Solution:
  def numRookCaptures(self, board: List[List[str]]) -> int:
    rows, cols = len(board), len(board[0])
    result = 0
    for row in range(rows):
      for col in range(cols):
        if board[row][col] != 'R':
          continue
        RRow, RCol = row, col
        for row in range(RRow - 1, -1, -1):
          if board[row][RCol] == '.': continue
          if board[row][RCol] == 'B': break
          if board[row][RCol] == 'p':
            result += 1
            break
        for row in range(RRow + 1, rows):
          if board[row][RCol] == '.': continue
          if board[row][RCol] == 'B': break
          if board[row][RCol] == 'p':
            result += 1
            break
        for col in range(RCol - 1, -1, -1):
          if board[RRow][col] == '.': continue
          if board[RRow][col] == 'B': break
          if board[RRow][col] == 'p':
            result += 1
            break
        for col in range(RCol + 1, cols):
          if board[RRow][col] == '.': continue
          if board[RRow][col] == 'B': break
          if board[RRow][col] == 'p':
            result += 1
            break
    return result

board = [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","R",".",".",".","p"],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
s = Solution()
result = s.numRookCaptures(board)
print(result)
