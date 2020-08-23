from typing import List

class Solution:
  def permutation(self, s: str) -> List[str]:
    result, slist = [], list(s)
    def dfs(x):
      if x == len(s):
        result.append(''.join(slist))
        return
      dic = set()
      for i in range(x, len(s)):
        if slist[i] in dic: continue
        dic.add(slist[i])
        slist[x], slist[i] = slist[i], slist[x]
        dfs(x + 1)
        slist[x], slist[i] = slist[i], slist[x]
    dfs(0)
    return result

s = Solution()
result = s.permutation("abb")
print(result)

