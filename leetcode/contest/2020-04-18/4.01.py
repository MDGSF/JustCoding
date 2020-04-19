# jump = [2, 5, 1, 1, 1, 1]
from typing import List

class Solution:
  def minJump(self, jump: List[int]) -> int:
    ctx = [float('inf')] * len(jump)
    ctx[0] = 0
    result = float('inf')
    for i in range(len(jump)):
      nexti = i + jump[i]
      if nexti >= len(jump):
        result = min(result, ctx[i] + 1)
        continue
      ctx[nexti] = min(ctx[nexti], ctx[i] + 1)
      for j in range(0, nexti):
        ctx[j] = min(ctx[j], ctx[i] + 2)
    return result

jump = [2, 5, 1, 1, 1, 1]
s = Solution()
result = s.minJump(jump)
print(result)
