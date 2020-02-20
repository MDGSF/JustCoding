from typing import List


class Solution:
  def plusOne(self, digits: List[int]) -> List[int]:
    carry = 1
    for i in range(len(digits) - 1, -1, -1):
      digits[i] += carry
      carry = digits[i] // 10
      digits[i] %= 10
    if carry > 0:
      digits.insert(0, carry)
    return digits


s = Solution()
result = s.plusOne([1,2,3])
print(result)
