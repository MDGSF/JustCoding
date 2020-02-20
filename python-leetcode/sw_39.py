class Solution:
  def majorityElement(self, nums: List[int]) -> int:
    result, count = nums[0], 1
    for i in range(1, len(nums)):
      if nums[i] == result:
        count += 1
      elif count == 1:
        result = nums[i]
      else:
        count -= 1
    return result
