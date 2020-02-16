import collections
from typing import List


class Solution:
  def findAnagrams(self, s: str, p: str) -> List[int]:
    result = []
    pCounter = collections.Counter(p)
    sCounter = collections.Counter(s[:len(p)])
    for i in range(len(s) - len(p) + 1):
      if sCounter == pCounter:
        result.append(i)
      if sCounter[s[i]] > 1:
        sCounter[s[i]] -= 1
      else:
        del sCounter[s[i]]
      if i + len(p) < len(s):
        sCounter[s[i + len(p)]] += 1
    return result

s = "cbaebabacd"
p = "abc"
solu = Solution()
result = solu.findAnagrams(s, p)
print(result)
