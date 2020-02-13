class Solution:
  def maxArea(self, height: List[int]) -> int:
    result = 0
    for i in range(len(height)-1):
      for j in range(i + 1, len(height)):
        curHeight = min(height[i], height[j])
        curWdith = j - i
        curArea = curWdith * curHeight
        result = max(result, curArea)
    return result

