class Solution:
  def findRepeatNumber(self, nums: List[int]) -> int:
    m = {}
    for num in nums:
      if num in m:
        return num
      else:
        m[num] = 1
    return -1

