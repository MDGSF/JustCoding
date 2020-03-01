class Solution:
  def isIsomorphic(self, s: str, t: str) -> bool:
    if len(s) != len(t): return False
    ms, mt, cs, ct = {}, {}, 0, 0
    for i in range(len(s)):
      if s[i] not in ms:
        cs += 1
        ms[s[i]] = cs
      if t[i] not in mt:
        ct += 1
        mt[t[i]] = ct
      if ms[s[i]] != mt[t[i]]:
        return False
    return True
