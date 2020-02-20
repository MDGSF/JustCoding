class Solution:
  def findNumbers(self, nums: List[int]) -> int:
    count = 0
    for num in nums:
      if self.isEven(num):
        count += 1
    return count

  def isEven(self, num):
    if 10 <= num < 100 or \
      1000 <= num < 10000 or \
      100000 <= num:
      return True
    return False
