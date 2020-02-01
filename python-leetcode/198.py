class Solution:
  def rob(self, nums: List[int]) -> int:
    f1, f2 = 0, 0
    for num in nums:
      f1, f2 = f2, max(f1 + num, f2)
    return f2

