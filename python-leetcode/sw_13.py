class Solution:
  def movingCount(self, m: int, n: int, k: int) -> int:
    result, queue, visited = 1, [(0, 0)], {(0, 0)}
    while queue:
      node = queue.pop(0)
      row, col = node[0], node[1]
      for x, y in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
        newRow, newCol = row + x, col + y
        if 0 <= newRow < m and 0 <= newCol < n \
            and (newRow, newCol) not in visited \
            and self.getNum(newRow) + self.getNum(newCol) <= k:
          result += 1
          visited.add((newRow, newCol))
          queue.append((newRow, newCol))
    return result

  def getNum(self, num):
    result = 0
    while num > 0:
      result += num % 10
      num = num // 10
    return result

m = 2
n = 3
k = 1

m = 3
n = 1
k = 0

s = Solution()
result = s.movingCount(m, n, k)
print(result)
