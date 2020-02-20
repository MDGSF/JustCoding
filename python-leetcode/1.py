class Solution:
  def twoSum(self, nums: List[int], target: int) -> List[int]:
    m = {}
    for i in range(len(nums)):
      num = nums[i]
      peer = target - num
      if peer in m:
        return [m[peer], i]
      m[num] = i
