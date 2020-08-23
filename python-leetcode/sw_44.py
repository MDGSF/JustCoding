class Solution:
  def findNthDigit(self, n: int) -> int:
    if n < 0: return -1
    digits = 1
    while (True):
      numbers = self.countOfIntegers(digits)
      if n < numbers * digits:
        return self.digitAtIndex(n, digits)
      n -= digits * numbers
      digits += 1
    return -1

  def countOfIntegers(self, digits):
    if digits == 1: return 10
    return 9 * pow(10, digits - 1)

  def beginNumber(self, digits):
    if digits == 1: return 0
    return pow(10, digits - 1)

  def digitAtIndex(self, n, digits):
    number = self.beginNumber(digits) + n // digits
    indexFromRight = digits - n % digits
    for i in range(1, indexFromRight):
      number //= 10
    return number % 10

s = Solution()
result = s.findNthDigit(11)
print(result)

