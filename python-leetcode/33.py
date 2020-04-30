class Solution:
  def search(self, nums: List[int], target: int) -> int:
    left, right = 0, len(nums) - 1
    while left <= right:
      mid = left + (right - left) // 2
      num = nums[mid] if (nums[mid] < nums[0]) == (target < nums[0]) \
        else (float('-inf') if target < nums[0] else float('+inf'))
      if num < target:
        left = mid + 1
      elif num > target:
        right = mid - 1
      else:
        return mid
    return -1
