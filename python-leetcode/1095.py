# """
# This is MountainArray's API interface.
# You should not implement it, or speculate about its implementation
# """
#class MountainArray:
#    def get(self, index: int) -> int:
#    def length(self) -> int:

class Solution:
  def findInMountainArray(self, target: int, mountain_arr: 'MountainArray') -> int:
    left, right = 0, mountain_arr.length() - 1
    while left < right:
      mid = (left + right) // 2
      if mountain_arr.get(mid) < mountain_arr.get(mid + 1):
        left = mid + 1
      else:
        right = mid
    peak = left
    index = self.binarySearch(mountain_arr, target, 0, peak)
    if index != -1:
      return index
    index = self.binarySearch(mountain_arr, target, peak + 1, mountain_arr.length() - 1, lambda x: -x)
    return index

  def binarySearch(self, mountain_arr, target, left, right, key = lambda x: x):
    target = key(target)
    while left <= right:
      mid = (left + right) // 2
      cur = key(mountain_arr.get(mid))
      if cur == target:
        return mid
      elif cur < target:
        left = mid + 1
      else:
        right = mid - 1
    return -1
