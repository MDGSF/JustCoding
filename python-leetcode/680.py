class Solution:
  def validPalindrome(self, s: str) -> bool:
    def dfs(left, right, canRemove):
      if left >= right:
        return True
      if s[left] == s[right]:
        return dfs(left + 1, right - 1, canRemove)
      elif canRemove:
        return dfs(left + 1, right, False) or dfs(left, right - 1, False)
      else:
        return False
    return dfs(0, len(s) - 1, True)

s = Solution()
result = s.validPalindrome("abc")
print(result)
