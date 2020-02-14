class Solution:
  def CheckPermutation(self, s1: str, s2: str) -> bool:
    if len(s1) != len(s2): return False
    m = [0] * 26
    for i in range(len(s1)):
      m[ord(s1[i]) - 97] += 1
      m[ord(s2[i]) - 97] -= 1
    return len(list(filter(lambda e: e > 0, m))) == 0

