class Solution:
  def isStraight(self, nums: List[int]) -> bool:
    joker = 0
    nums.sort()
    for i in range(4):
      if nums[i] == 0: joker += 1 # 统计大小王数量
      elif nums[i] == nums[i+1]: return False # 若有重复，提前返回 False
    return nums[4] - nums[joker] < 5 # 最大牌 - 最小牌 < 5 则可以构成顺子

