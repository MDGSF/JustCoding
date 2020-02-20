class Solution:
  def removeDuplicates(self, nums: List[int]) -> int:
    lastUniqIdx = 0
    for i in range(1, len(nums)):
      if nums[i] != nums[lastUniqIdx]:
        lastUniqIdx += 1
        nums[lastUniqIdx] = nums[i]
    return lastUniqIdx + 1
