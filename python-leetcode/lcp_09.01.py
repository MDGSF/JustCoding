from typing import List

# bfs，会超时
class Solution:
  def minJump(self, jump: List[int]) -> int:
    s = {0}
    visited = set()
    count = 0
    while s:
      nexts = set()
      while s:
        idx = s.pop()
        if idx + jump[idx] >= len(jump):
          return count + 1
        nexts.add(idx + jump[idx])
        visited.add(idx + jump[idx])
        for i in range(idx):
          if i not in visited:
            nexts.add(i)
            visited.add(i)
      s = nexts
      count += 1
    return count

jump = [2, 5, 1, 1, 1, 1]
s = Solution()
result = s.minJump(jump)
print(result)
