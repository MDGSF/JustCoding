class Solution:
  def permute(self, nums: List[int]) -> List[List[int]]:
    result = []
    def dfs(first):
      if first == len(nums):
        result.append(nums.copy())
        return
      for i in range(first, len(nums)):
        nums[first], nums[i] = nums[i], nums[first]
        dfs(first + 1)
        nums[first], nums[i] = nums[i], nums[first]
    dfs(0)
    return result
