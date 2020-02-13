from typing import List

class Solution:
  def maxArea(self, height: List[int]) -> int:
    result, left, right = 0, 0, len(height)-1
    while left < right:
      curHeight = min(height[left], height[right])
      curWdith = right - left
      curArea = curWdith * curHeight
      result = max(result, curArea)
      if height[left] < height[right]: left += 1
      else: right -= 1
    return result

s = Solution()
result = s.maxArea([1,8,6,2,5,4,8,3,7])
print(result)
