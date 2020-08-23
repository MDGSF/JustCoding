class Solution:
  def countDigitOne(self, n: int) -> int:
    digit, res = 1, 0
    high, cur, low = n // 10, n % 10, 0
    while high != 0 or cur != 0:
      if cur == 0: res += high * digit
      elif cur == 1: res += high * digit + low + 1
      else: res += (high + 1) * digit
      low += cur * digit
      cur = high % 10
      high //= 10
      digit *= 10
    return res

s = Solution()
result = s.countDigitOne(13)
print(result)

