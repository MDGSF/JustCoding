class Solution:
  def countSubstrings(self, s: str) -> int:
    def manachers(s):
      s = '@#' + '#'.join(s) + '#$'
      p, n, idx, mx = [0] * len(s), len(s), 0, 0
      for i in range(1, n - 1):
        if mx > i:
          p[i] = min(p[2*idx-i], mx - i)
        while s[i + p[i] + 1] == s[i - p[i] - 1]:
          p[i] += 1
        if i + p[i] > mx:
          idx, mx = i, i + p[i]
      return p
    p = manachers(s)
    return sum((v + 1) // 2 for v in p)

s = "abc"
solu = Solution()
result = solu.countSubstrings(s)
print(result)
