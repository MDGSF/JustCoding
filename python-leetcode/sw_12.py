from typing import List

class Solution:
  def exist(self, board: List[List[str]], word: str) -> bool:
    def dfs(row, col, i):
      if not 0 <= row < len(board) or \
          not 0 <= col < len(board[0]) or \
          board[row][col] != word[i]:
            return False
      if i == len(word) - 1:
        return True
      tmp, board[row][col] = board[row][col], '/'
      result = dfs(row + 1, col, i + 1) or \
          dfs(row - 1, col, i + 1) or \
          dfs(row, col + 1, i + 1) or \
          dfs(row, col - 1, i + 1)
      board[row][col] = tmp
      return result
    for row in range(len(board)):
      for col in range(len(board[0])):
        if dfs(row, col, 0):
          return True
    return False

board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]
word = "ABCCED"
s = Solution()
result = s.exist(board, word)
print(result)

