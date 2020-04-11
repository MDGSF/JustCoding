class Solution:
  def sortArray(self, nums: List[int]) -> List[int]:
    self.quickSort(nums, 0, len(nums) - 1)
    return nums

  def quickSort(self, nums, start, end):
    if start < end:
      partitionIndex = self.partition(nums, start, end)
      self.quickSort(nums, start, partitionIndex - 1)
      self.quickSort(nums, partitionIndex + 1, end)

  def partition(self, nums, start, end):
    pivot = nums[end]
    partitionIndex = start
    for i in range(start, end):
      if nums[i] < pivot:
        nums[i], nums[partitionIndex] = nums[partitionIndex], nums[i]
        partitionIndex += 1
    nums[partitionIndex], nums[end] = nums[end], nums[partitionIndex]
    return partitionIndex
