class Solution:
  def trap(self, height: List[int]) -> int:
    i, j, result, bucket = 0, len(height) - 1, 0, 0
    while i < j:
      block = min(height[i], height[j])
      bucket = max(bucket, block)
      if height[i] < height[j]:
        result += bucket - height[i]
        i += 1
      else:
        result += bucket - height[j]
        j -= 1
    return result

