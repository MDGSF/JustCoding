class Solution:
  def maxProfit(self, prices: List[int]) -> int:
    minprice, maxprofit = float('inf'), 0
    for price in prices:
      if price < minprice:
        minprice = price
      elif price - minprice > maxprofit:
        maxprofit = price - minprice
    return maxprofit

