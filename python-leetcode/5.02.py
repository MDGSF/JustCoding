# 暴力法，两重循环确定左右边界，再一重循环判断是否是回文串
# 时间复杂度 O(n^3)，会超时
class Solution:
  def longestPalindrome(self, s: str) -> str:
    maxs, maxlen = "", float('-inf')
    for i in range(len(s)):
      for j in range(i, len(s)):
        curs = s[i:j+1]
        if self.isPalindrome(curs) and j+1 - i > maxlen:
          maxs, maxlen = curs, j+1 - i
    return maxs

  def isPalindrome(self, s):
    i, j = 0, len(s) - 1
    while i < j:
      if s[i] != s[j]:
        return False
      i += 1
      j -= 1
    return True

s = "hello"
s = "121"
s = "babad"
s = "a"
s = "cbbd"
solu = Solution()
result = solu.longestPalindrome(s)
print(result)
