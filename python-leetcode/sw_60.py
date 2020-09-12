from typing import List

class Solution:
  def twoSum(self, n: int) -> List[float]:
    pre = [1/6] * 6
    for i in range(2, n + 1):
      tmp = [0] * (5 * i + 1)
      for j in range(0, len(pre)):
        for x in range(0, 6):
          tmp[j + x] += pre[j] / 6
      pre = tmp
    return pre

s = Solution()
result = s.twoSum(2)
print(result)

# 1 2 3 4 5 6

# 2 个骰子
# 2 ~ 12
# 6^2 = 36
#
# 2 = 1 + 1
# 3 = 1 + 2, 2 + 1
# 4 = 2 + 2, 1 + 3, 3 + 1
# 5 = 1 + 4, 4 + 1, 2 + 3, 3 + 2
# 6 = 1 + 5, 5 + 1, 2 + 4, 4 + 2, 3 + 3
# 7 = 1 + 6, 6 + 1, 2 + 5, 5 + 2, 3 + 4, 4 + 3
# 8 = 2 + 6, 6 + 2, 3 + 5, 5 + 3, 4 + 4
# 9 = 3 + 6, 6 + 3, 4 + 5, 5 + 4
# 10 = 4 + 6, 6 + 4, 5 + 5
# 11 = 5 + 6, 6 + 5
# 12 = 6 + 6

