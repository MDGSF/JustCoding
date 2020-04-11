class Solution:
  def lastRemaining(self, n: int, m: int) -> int:
    if n == 0: return 0
    x = self.lastRemaining(n - 1, m)
    return (x + m) % n

n = 5
m = 3
s = Solution()
result = s.lastRemaining(n, m)
print(result)

