class Solution:
  def gameOfLife(self, board: List[List[int]]) -> None:
    """
    Do not return anything, modify board in-place instead.
    """
    self.rows, self.cols = len(board), len(board[0])
    for row in range(self.rows):
      for col in range(self.cols):
        liveNeighborsNum = self.calcLiveNeighborsNum(board, row, col)
        if board[row][col] == 1 and liveNeighborsNum < 2:
          board[row][col] = -1 # 过去是活的，现在死了
        elif board[row][col] == 1 and 2 <= liveNeighborsNum <= 3:
          board[row][col] = 1 # 过去是活的，现在还是活的
        elif board[row][col] == 1 and liveNeighborsNum > 3:
          board[row][col] = -1 # 过去是活的，现在死了
        elif board[row][col] == 0 and liveNeighborsNum == 3:
          board[row][col] = 2 # 过去是死的，现在活了
    for row in range(self.rows):
      for col in range(self.cols):
        if board[row][col] > 0:
          board[row][col] = 1
        else:
          board[row][col] = 0

  def calcLiveNeighborsNum(self, board, row, col):
    count = 0
    neighbors = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]]
    for neighbor in neighbors:
      newRow, newCol = row + neighbor[0], col + neighbor[1]
      if self.isValid(newRow, newCol) and abs(board[newRow][newCol]) == 1:
        count += 1
    return count

  def isValid(self, row, col):
    return 0 <= row < self.rows and 0 <= col < self.cols
