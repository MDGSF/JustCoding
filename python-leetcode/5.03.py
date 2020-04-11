# 中心向两端扩展法，一重遍历所有中心点，再一重循环从中心点向两端扩展
# 时间复杂度 O(n^2)
class Solution:
  def longestPalindrome(self, s: str) -> str:
    n, maxs, maxlen = len(s), "", float('-inf')
    for i in range(n): # i 为回文串中心位置
      # 回文串长度为奇数
      j, curStart, curlen = 0, 0, 0
      while i - j >= 0 and i + j < n:
        if s[i - j] != s[i + j]:
          break
        curStart, curlen = i - j, j * 2 + 1
        j += 1
      if curlen > maxlen:
        maxlen = curlen
        maxs = s[curStart:curStart+curlen]
      # 回文串长度为偶数
      j, curStart, curlen = 0, 0, 0
      while i - j >= 0 and i + j + 1 < n:
        if s[i - j] != s[i + j + 1]:
          break
        curStart, curlen = i - j, j * 2 + 2
        j += 1
      if curlen > maxlen:
        maxlen = curlen
        maxs = s[curStart:curStart+curlen]
    return maxs

s = "hello"
s = "121"
s = "babad"
s = "cbbd"
s = "a"
solu = Solution()
result = solu.longestPalindrome(s)
print(result)
