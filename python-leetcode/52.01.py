class Solution:
  def totalNQueens(self, n: int) -> int:
    self.count = 0
    def dfs(cols, pie, na, row):
      if row >= n:
        self.count += 1
        return
      for col in range(n):
        if (col not in cols) and (row + col not in pie) and (row - col not in na):
          dfs(cols|{col}, pie|{row+col}, na|{row-col}, row+1)
    dfs(set(), set(), set(), 0)
    return self.count

s = Solution()
result = s.totalNQueens(8)
print(result)
