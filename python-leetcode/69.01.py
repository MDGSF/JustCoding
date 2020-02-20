class Solution:
  # 牛顿迭代法
  def mySqrt(self, x: int) -> int:
    if x <= 1: return x
    r = x
    while r * r > x:
      r = (r + x // r) // 2
    return int(r)
