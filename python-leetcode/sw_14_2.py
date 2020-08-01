class Solution:
  def cuttingRope(self, n: int) -> int:
    if n <= 3:
      return n - 1
    quotient, remainder = n // 3, n % 3
    if remainder == 0:
      return 3 ** quotient % 1000000007
    elif remainder == 1:
      return 3 ** (quotient - 1) * 4 % 1000000007
    else:
      return 3 ** quotient * 2 % 1000000007

n = 2

s = Solution()
result = s.cuttingRope(n)
print(result)

