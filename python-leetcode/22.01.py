class Solution:
  def generateParenthesis(self, n: int) -> List[str]:
    result = []
    def dfs(left, right, node):
      if left == n and right == n:
        result.append(node)
      if left < n:
        dfs(left + 1, right, node + "(")
      if right < left:
        dfs(left, right + 1, node + ")")
    dfs(0, 0, "")
    return result
