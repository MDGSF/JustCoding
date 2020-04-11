class Solution:
  def movingCount(self, m: int, n: int, k: int) -> int:
    self.result, visited = 0, set()

    def dfs(row, col):
      if not isValid(row, col):
        return
      self.result += 1
      visited.add((row, col))
      for x, y in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
        newRow, newCol = row + x, col + y
        dfs(newRow, newCol)

    def isValid(row, col):
      if 0 <= row < m and 0 <= col < n \
        and (row, col) not in visited \
        and getNum(row) + getNum(col) <= k:
        return True
      return False

    def getNum(num):
      result = 0
      while num > 0:
        result += num % 10
        num = num // 10
      return result

    dfs(0, 0)
    return self.result

m = 2
n = 3
k = 1

# m = 3
# n = 1
# k = 0

s = Solution()
result = s.movingCount(m, n, k)
print(result)
