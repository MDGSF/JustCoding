class Solution:
  def mySqrt(self, x: int) -> int:
    return int(self.MySqrt(x, 0.01))

  def MySqrt(self, x, precision):
    if x == 0 or x == 1: return x
    left, right = 0, x
    while left < right:
      mid = left + (right - left) / 2
      cur = mid * mid
      if abs(cur - x) <= precision:
        return mid
      elif cur > x:
        right = mid
      else:
        left = mid
