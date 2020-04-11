from typing import List
from functools import reduce
import collections
import math

class Solution:
  def hasGroupsSizeX(self, deck: List[int]) -> bool:
    vals = collections.Counter(deck).values()
    return reduce(math.gcd, vals) >= 2

deck = [1,2,3,4,4,3,2,1] # True
deck = [1,1,1,2,2,2,3,3] # False
deck = [1] # False
deck = [1, 1] # True
deck = [1,1,2,2,2,2] # True
deck = [1,1,1,1,2,2,2,2,2,2]
solu = Solution()
result = solu.hasGroupsSizeX(deck)
print(result)

