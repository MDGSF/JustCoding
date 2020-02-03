from typing import List

class Solution:
  def reversePairs(self, nums: List[int]) -> int:
    self.temp = [0 for _ in range(len(nums))]
    return self.mergeSort(nums, 0, len(nums) - 1)

  def mergeSort(self, nums, start, end) -> int:
    if start >= end: return 0
    mid = start + (end - start) // 2
    count = self.mergeSort(nums, start, mid) + self.mergeSort(nums, mid+1, end)
    j = mid + 1
    for i in range(start, mid + 1):
      while j <= end and nums[i] > nums[j]*2:
        j += 1
      count += j - (mid + 1)
    # nums[start:end+1] = sorted(nums[start:end+1])
    self.merge(nums, start, mid, end)
    return count

  def merge(self, nums, left, mid, right):
    curLen = (right - left + 1)
    i, j, k = left, mid + 1, 0
    while i <= mid and j <= right:
      if nums[i] < nums[j]:
        self.temp[k] = nums[i]
        k += 1
        i += 1
      else:
        self.temp[k] = nums[j]
        k += 1
        j += 1
    while i <= mid:
      self.temp[k] = nums[i]
      k += 1
      i += 1
    while j <= right:
      self.temp[k] = nums[j]
      k += 1
      j += 1
    for p in range(curLen):
      nums[left + p] = self.temp[p]

s = Solution()
result = s.reversePairs([2,4,3,5,1])
print(result)

