class Solution:
  def majorityElement(self, nums: List[int]) -> int:
    result, count = nums[0], 1
    for num in nums:
      if result == num:
        count += 1
      else:
        if count == 1:
          result = num
        else:
          count -= 1
    return result
