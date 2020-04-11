# 暴力法2
class Solution:
  def strStr(self, haystack: str, needle: str) -> int:
    i, j, sLen, pLen = 0, 0, len(haystack), len(needle)
    while i < sLen and j < pLen:
      if haystack[i] == needle[j]:
        i += 1
        j += 1
      else: # 匹配失败的时候，i 回退， j 重置
        i = i - j + 1
        j = 0
    return i - j if j == pLen else -1

haystack = "a"
needle = "a"
# haystack = "hello"
# needle = "ll"
s = Solution()
result = s.strStr(haystack, needle)
print(result)
