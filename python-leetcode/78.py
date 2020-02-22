class Solution:
  def subsets(self, nums: List[int]) -> List[List[int]]:
    result = []
    def dfs(first, node):
      result.append(node)
      for i in range(first, len(nums)):
        dfs(i + 1, node + [nums[i]])
    dfs(0, [])
    return result
