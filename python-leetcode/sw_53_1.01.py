class Solution:
  def search(self, nums: List[int], target: int) -> int:
    counter = collections.Counter(nums)
    return counter[target]
