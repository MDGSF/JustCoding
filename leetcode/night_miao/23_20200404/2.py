import collections
class Solution:
  def canConstruct(self, s: str, k: int) -> bool:
    if k > len(s): return False
    if k == len(s): return True
    counter = collections.Counter(s)
    oddNum = 0
    for val in counter.values():
      if val % 2 != 0:
        oddNum += 1
    if oddNum > k:
      return False
    return True

s = "annabelle"
k = 2

s = "leetcode"
k = 3

s = "true"
k = 4

s = "yzyzyzyzyzyzyzy"
k = 2

s = "cr"
k = 7

solu = Solution()
result = solu.canConstruct(s, k)
print(result)
