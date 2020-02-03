class Solution:
  # 暴力法：遍历所有可能性，会超时
  def reversePairs(self, nums: List[int]) -> int:
    count = 0
    for i in range(len(nums)):
      for j in range(i+1, len(nums)):
        if nums[i] > 2 * nums[j]:
          count += 1
    return count

