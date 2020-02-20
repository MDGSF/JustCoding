class Solution:
  def numWays(self, n: int) -> int:
    if n == 0: return 1
    if n < 3: return n
    f1, f2 = 1, 2
    for _ in range(2, n):
      f1, f2 = f2, (f1 + f2) % 1000000007
    return f2
