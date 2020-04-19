
# Sa = "acb", Na = 4, Ra = "acbacbacbacb"
# Sb = "ab" , Nb = 2, Rb = "abab"
class Solution:
  def getMaxRepetitions(self, s1: str, n1: int, s2: str, n2: int) -> int:
    len1, len2 = len(s1), len(s2)
    if len1 == 0 or len2 == 0 or len1 * n1 < len2 * n2:
      return 0
    i, j = 0, 0 # Ra 和 Rb 的下标
    m1, m2 = {}, {}
    result = 0
    while i // len1 < n1:
      if i % len1 == len1 - 1:
        if (j % len2) in m1: # 出现循环
          val = m1[j % len2]
          cycleLen = i // len1 - val // len1
          cycleNum = (n1 - 1 - i // len1) // cycleLen
          cycleS2Num = j // len2 - m2[j % len2] // len2
          i += cycleNum * cycleLen * len1
          result += cycleNum * cycleS2Num
        else: # 第一次
          m1[j % len2] = i
          m2[j % len2] = j
      if s1[i % len1] == s2[j % len2]:
        if j % len2 == len2 - 1:
          result += 1
        j += 1
      i += 1
    return result // n2

s1 ="acb"
n1 = 4
s2 ="ab"
n2 = 2
s = Solution()
result = s.getMaxRepetitions(s1, n1, s2, n2)
print(result)
