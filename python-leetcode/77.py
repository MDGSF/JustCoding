class Solution:
  def combine(self, n: int, k: int) -> List[List[int]]:
    result = []
    def dfs(first, node):
      if len(node) == k:
        result.append(node)
        return
      for i in range(first, n + 1):
        dfs(i + 1, node + [i])
    dfs(1, [])
    return result
