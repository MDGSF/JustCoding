from typing import List


class Solution:
  def maxCoins(self, nums: List[int]) -> int:
    nums = [1] + [num for num in nums if num > 0] + [1]
    n = len(nums)
    m = {}
    def dfs(left, right):
      if left + 1 == right: return 0
      if (left, right) in m: return m[(left, right)]
      result = 0
      for i in range(left + 1, right):
        sub1 = dfs(left, i)
        sub2 = dfs(i, right)
        cur = nums[left] * nums[i] * nums[right] + sub1 + sub2
        result = max(result, cur)
      m[(left, right)] = result
      return result
    return dfs(0, n - 1)


nums = [3, 1, 5, 8]
s = Solution()
result = s.maxCoins(nums)
print(result)
