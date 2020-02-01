from typing import List

class Solution:
  def letterCombinations(self, digits: str) -> List[str]:
    m = {
        '2': ['a', 'b', 'c'],
        '3': ['d', 'e', 'f'],
        '4': ['g', 'h', 'i'],
        '5': ['j', 'k', 'l'],
        '6': ['m', 'n', 'o'],
        '7': ['p', 'q', 'r', 's'],
        '8': ['t', 'u', 'v'],
        '9': ['w', 'x', 'y', 'z'],
        }
    if len(digits) == 0:
      return []
    result = []
    def dfs(node, digits):
      if len(digits) == 0:
        result.append(node)
        return
      letters = m[digits[0]]
      for letter in letters:
        dfs(node + letter, digits[1:])
    dfs("", digits)
    return result

s = Solution()
result = s.letterCombinations("23")
print(result)
