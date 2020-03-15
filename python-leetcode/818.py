import collections
import math

class Solution:
  def __init__(self):
    self.dp = [0] * 10001

  def racecar(self, target: int) -> int:
    if self.dp[target] > 0: return self.dp[target]
    n = math.floor(math.log2(target)) + 1
    if target + 1 == (1<<n):
      self.dp[target] = n
      return self.dp[target]
    # n 个 A 到达 2^n-1 位置，然后 R 反向，走完剩余
    # n + 1 就是 n 个 A 和一个 R
    # (1<<n) - 1 - target 就是 2^n-1 和 target 之间的距离
    self.dp[target] = n + 1 + self.racecar((1<<n) - 1 - target)
    # n-1 个 A 到达 2^(n-1)-1 位置，然后 R 反向走 m 个 A，再 R 反向，走完剩余
    # m 取值遍历 [0, n-1)
    for m in range(n - 1):
      cur = (n-1) + 1 + m + 1 + self.racecar(target - (1<<(n-1)) + (1<<m))
      self.dp[target] = min(self.dp[target], cur)
    return self.dp[target]


target = 3 # 2
target = 6 # 5
target = 5 # 7

s = Solution()
result = s.racecar(5)
print(result)
