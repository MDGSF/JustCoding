class Solution:

  def xorOperation(self, n: int, start: int) -> int:
    result = 0
    for i in range(n):
      result = result ^ (start + 2 * i)
    return result

s = Solution()
# result = s.xorOperation(5, 0)
# result = s.xorOperation(4, 3)
result = s.xorOperation(1, 7)
print(result)
