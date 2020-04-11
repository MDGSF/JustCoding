class Solution:
  def sortArray(self, nums: List[int]) -> List[int]:
    self.mergeSort(nums, 0, len(nums) - 1)
    return nums

  def mergeSort(self, nums, left, right):
    if left < right:
      mid = (left + right) // 2
      self.mergeSort(nums, left, mid)
      self.mergeSort(nums, mid + 1, right)
      self.merge(nums, left, mid, right)

  def merge(self, nums, left, mid, right):
    temp = [0] * (right - left + 1)
    i, j, k = left, mid + 1, 0
    while i <= mid and j <= right:
      if nums[i] < nums[j]:
        temp[k] = nums[i]
        i += 1
        k += 1
      else:
        temp[k] = nums[j]
        j += 1
        k += 1
    while i <= mid:
      temp[k] = nums[i]
      i += 1
      k += 1
    while j <= right:
      temp[k] = nums[j]
      j += 1
      k += 1
    for p in range(len(temp)):
      nums[left + p] = temp[p]
