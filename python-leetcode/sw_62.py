class Solution:
  def lastRemaining(self, n: int, m: int) -> int:
    last = 0
    for i in range(2, n + 1):
      last = (last + m) % i
    return last

n = 5
m = 3
s = Solution()
result = s.lastRemaining(n, m)
print(result)

