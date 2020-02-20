class Solution:
  def trap(self, height: List[int]) -> int:
    result, stack = 0, []
    for i in range(len(height)):
      while stack and height[i] > height[stack[-1]]:
        tmp = stack.pop()
        if len(stack) == 0:
          break
        curWidth = i - stack[-1] - 1
        curHeight = min(height[i], height[stack[-1]]) - height[tmp]
        curArea = curWidth * curHeight
        result += curArea
      stack.append(i)
    return result
