class Solution:
  def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
    if len(nums) == 0: return []
    result, window = [], []
    for i in range(len(nums)):
      if i >= k and i - k >= window[0]:
        window.pop(0)
      while len(window) > 0 and nums[i] > nums[window[-1]]:
        window.pop()
      window.append(i)
      if i >= k - 1:
        result.append(nums[window[0]])
    return result
