class Solution:
  def isHappy(self, n: int) -> bool:
    s = set()
    while n != 1 and n not in s:
      s.add(n)
      n = self.getNext(n)
    return n == 1

  def getNext(self, n):
    total = 0
    while n > 0:
      n, digit = divmod(n, 10)
      total += digit ** 2
    return total
