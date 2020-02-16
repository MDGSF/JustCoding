class Solution:

  def isMatch(self, s: str, p: str) -> bool:
    if len(p) == 0:
      return len(s) == 0
    if p[0] == '*':
      return self.isMatch(s, p[1:]) or (len(s) != 0) and self.isMatch(s[1:], p)
    else:
      firstMatch = (len(s) != 0) and (p[0] == s[0] or p[0] == '?')
      return firstMatch and self.isMatch(s[1:], p[1:])


s = "acdcb"
p = "a*c?b"
solu = Solution()
result = solu.isMatch(s, p)
print(result)
