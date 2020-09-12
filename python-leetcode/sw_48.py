class Solution:
  def lengthOfLongestSubstring(self, s: str) -> int:
    m, result, left = {}, 0, -1
    for right in range(len(s)):
      if s[right] in m: left = max(m[s[right]], left)
      m[s[right]] = right
      result = max(result, right - left)
    return result

s = "abcabcbb"
solu = Solution()
result = solu.lengthOfLongestSubstring(s)
print(result)

