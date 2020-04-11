
# Manacher 马拉车算法
# 时间复杂度 O(n)
class Solution:
  def longestPalindrome(self, s: str) -> str:
    s = '$#' + '#'.join(list(s)) + '#'
    p, n, idx, mx = [0] * len(s), len(s), 0, 0
    for i in range(1, n):
      if mx > i:
        p[i] = min(p[2*idx-i], mx - i)
      else:
        p[i] = 1
      while i + p[i] < n and i - p[i] >=0 and s[i + p[i]] == s[i - p[i]]:
        p[i] += 1
      if i + p[i] > mx:
        mx = i + p[i]
        idx = i
    maxPalindromeLen = float('-inf')
    maxIdx = -1
    for i in range(len(p)):
      if p[i] > maxPalindromeLen:
        maxPalindromeLen = p[i]
        maxIdx = i
    maxPalindromeLen -= 1
    result = s[maxIdx - maxPalindromeLen + 1:maxIdx + maxPalindromeLen]
    return result.replace('#', '')

s = "hello"
s = "121"
s = "babad"
solu = Solution()
result = solu.longestPalindrome(s)
print(result)
