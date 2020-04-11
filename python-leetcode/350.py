import collections

class Solution:
  def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
    m = collections.Counter(nums1)
    result = []
    for num in nums2:
      if num in m:
        result.append(num)
        if m[num] == 1:
          del m[num]
        else:
          m[num] -= 1
    return result
