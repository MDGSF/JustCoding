# 暴力法1
class Solution:
  def strStr(self, haystack: str, needle: str) -> int:
    if len(needle) == 0: return 0
    for i in range(len(haystack) - len(needle) + 1):
      matches = True
      for j in range(len(needle)):
        if haystack[i + j] != needle[j]:
          matches = False
          break
      if matches:
        return i
    return -1

haystack = "a"
needle = "a"
# haystack = "hello"
# needle = "ll"
s = Solution()
result = s.strStr(haystack, needle)
print(result)
