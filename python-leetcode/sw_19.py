class Solution:
  def isMatch(self, s: str, p: str) -> bool:
    m = {}
    def dfs(i, j):
      if (i, j) in m:
        return m[(i, j)]
      if len(p) == j:
        return len(s) == i
      firstMatch = (i < len(s)) and (p[j] == s[i] or p[j] == '.')
      if len(p) - 2 >= j and p[j + 1] == '*':
        result = dfs(i, j + 2) or firstMatch and dfs(i + 1, j)
      else:
        result = firstMatch and dfs(i + 1, j + 1)
      m[(i, j)] = result
      return result
    return dfs(0, 0)

s = Solution()
result = s.isMatch("aa", "a*")
print(result)
