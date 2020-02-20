class Solution:
  def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
    s1 = set(nums1)
    s2 = set(nums2)
    return s1.intersection(s2)
