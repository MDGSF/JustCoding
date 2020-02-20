class Solution:
  # 等差数列求和公式
  def missingNumber(self, nums: List[int]) -> int:
    n = len(nums) + 1
    s = n * (n - 1) / 2
    return int(s - sum(nums))
