class Solution:

  def climbStairs(self, n: int) -> int:
    if n < 3:
      return n
    f1, f2 = 1, 2
    for i in range(2, n):
      f1, f2 = f2, f1 + f2
    return f2

s = Solution()
for i in range(1, 10):
  print(s.climbStairs(i));

