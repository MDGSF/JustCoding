class Solution:
  def countLargestGroup(self, n: int) -> int:
    m = {}
    for i in range(1, n + 1):
      s = self.calc(i)
      if s in m:
        m[s] += 1
      else:
        m[s] = 1
    maxVal = max(m.values())
    count = 0
    for val in m.values():
      if val == maxVal:
        count += 1
    return count

  def calc(self, n):
    s = 0
    while n > 0:
      s += n % 10
      n = n // 10
    return s

s = Solution()
result = s.countLargestGroup(15)
print(result)
