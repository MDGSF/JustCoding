class Solution:
  def convertToTitle(self, n: int) -> str:
    result = []
    while n > 0:
      n -= 1
      quotient = n // 26
      remainder = n % 26
      n = quotient
      result.append(remainder)
    return ''.join(list(map(lambda e: chr(ord('A') + e), result))[::-1])

s = Solution()
result = s.convertToTitle(52)
print(result)
