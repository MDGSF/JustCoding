from typing import List

class Solution:
  def minCount(self, coins: List[int]) -> int:
    result = 0
    for coin in coins:
      result += coin // 2
      if coin % 2 != 0:
        result += 1
    return result

coins = [4,2,1]
coins = [2, 3, 10]
s = Solution()
result = s.minCount(coins)
print(result)

