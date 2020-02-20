class Solution:
  def findMin(self, nums: List[int]) -> int:
    left, right, mid = 0, len(nums) - 1, 0
    while nums[left] >= nums[right]:
      if right - left == 1:
        mid = right
        break
      mid = (left + right) // 2
      if nums[left] == nums[mid] and nums[mid] == nums[right]:
        return min(nums[left:right+1])
      elif nums[left] <= nums[mid]:
        left = mid
      elif nums[mid] <= nums[right]:
        right = mid
    return nums[mid]
