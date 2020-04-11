class Solution:
  # K: 鸡蛋数目
  # N: 楼层数
  # dp(K, N)
  # dp(K, N) = 1 + min[0<=i<=N]( max(dp(K, N-i), dp(K-1, i-1)) )
  # 把线性查找改为二分查找
  def superEggDrop(self, K: int, N: int) -> int:
    m = {}
    def dfs(K, N):
      if K == 1: return N
      if N == 0: return 0
      if (K, N) in m: return m[(K, N)]
      result = float('inf')
      left, right = 1, N
      while left <= right:
        mid = (left + right) // 2
        sub1 = dfs(K, N - mid) # 鸡蛋没碎
        sub2 = dfs(K - 1, mid - 1) # 鸡蛋碎了
        cur = max(sub1, sub2) + 1
        result = min(result, cur)
        if sub1 > sub2:
          left = mid + 1
        else:
          right = mid - 1
      m[(K, N)] = result
      return result
    return dfs(K, N)
