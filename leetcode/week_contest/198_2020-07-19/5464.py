class Solution:
  def numWaterBottles(self, numBottles: int, numExchange: int) -> int:
    result = numBottles
    left = numBottles
    while left >= numExchange:
      cur = left // numExchange
      result += cur
      left %= numExchange
      left += cur
    return result

numBottles = 15
numExchange = 4

numBottles = 5
numExchange = 5

numBottles = 2
numExchange = 3

s = Solution()
result = s.numWaterBottles(numBottles, numExchange)
print(result)

