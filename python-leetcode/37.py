from typing import List
class Solution:
  def solveSudoku(self, board: List[List[str]]) -> None:
    """
    Do not return anything, modify board in-place instead.
    """
    self.board = board
    self.rows = len(board)
    self.cols = len(board[0])
    self.solve()

  def solve(self):
    for row in range(self.rows):
      for col in range(self.cols):
        if self.board[row][col] != ".":
          continue
        for num in ["1","2","3","4","5","6","7","8","9"]:
          if self.isValid(row, col, num):
            self.board[row][col] = num
            if self.solve():
              return True
            self.board[row][col] = "."
        return False
    return True

  def isValid(self, row, col, c):
    for i in range(9):
      if self.board[i][col] == c:
        return False
      if self.board[row][i] == c:
        return False
      if self.board[row // 3 * 3 + i // 3][col // 3 * 3 + i % 3] == c:
        return False
    return True

board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]

s = Solution()
s.solveSudoku(board)
print(board)
