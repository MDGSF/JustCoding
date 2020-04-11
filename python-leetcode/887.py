class Solution:
  # dp[k][m] 表示当前有 k 个鸡蛋，可以尝试扔 m 次鸡蛋
  # 这个状态下，最坏情况下能够测试一栋 n 层的楼
  #
  # dp[k][m] = dp[k][m-1] + dp[k-1][m-1] + 1
  # dp[k][m-1] 鸡蛋没碎，表示楼上的楼层
  # dp[k-1][m-1] 鸡蛋碎了，表示楼下的楼层
  def superEggDrop(self, K: int, N: int) -> int:
    dp = [[0 for _ in range(N + 1)] for _ in range(K + 1)]
    m = 0
    while dp[K][m] < N:
      m += 1
      for k in range(1, K + 1):
        dp[k][m] = dp[k][m - 1] + dp[k - 1][m - 1] + 1
    return m

K = 3
N = 14

s = Solution()
result = s.superEggDrop(K, N)
print(result)
