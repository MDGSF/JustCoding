from typing import List
import collections

class Solution:
  def permuteUnique(self, nums: List[int]) -> List[List[int]]:
    result = []
    counter = collections.Counter(nums)
    def dfs(node):
      if len(node) == len(nums):
        result.append(node.copy())
        return
      for num in counter.keys():
        if counter[num] > 0:
          counter[num] -= 1
          node.append(num)
          dfs(node)
          node.pop()
          counter[num] += 1
    dfs([])
    return result

s = Solution()
result = s.permuteUnique([1,1,2])
print(result)
