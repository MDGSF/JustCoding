class Solution:
  def numSteps(self, s: str) -> int:
    i = int(s, 2)
    count = 0
    while i != 1:
      if i % 2 == 0:
        i = i // 2
      else:
        i = i + 1
      count += 1
    return count


s = "1"
solu = Solution()
result = solu.numSteps(s)
print(result)
