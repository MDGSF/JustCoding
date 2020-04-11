# KMP
class Solution:
  def strStr(self, haystack: str, needle: str) -> int:
    if len(needle) == 0: return 0
    i, j, sLen, pLen = 0, 0, len(haystack), len(needle)
    next = self.getNext(needle)
    while i < sLen and j < pLen:
      if j == -1 or haystack[i] == needle[j]:
        i += 1
        j += 1
      else:
        j = next[j]
    return i - j if j == pLen else -1

  def getNext(self, needle):
    pLen = len(needle)
    next = [0] * pLen
    next[0] = -1
    k = -1
    j = 0
    while j < pLen - 1:
      if k == -1 or needle[j] == needle[k]:
        j += 1
        k += 1
        if needle[j] != needle[k]:
          next[j] = k
        else:
          next[j] = next[k]
      else:
        k = next[k]
    return next

# haystack = "a"
# needle = "a"
haystack = "hello"
needle = "ll"
s = Solution()
result = s.strStr(haystack, needle)
print(result)
