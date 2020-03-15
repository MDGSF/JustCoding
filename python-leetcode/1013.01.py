class Solution:
  def canThreePartsEqualSum(self, A: List[int]) -> bool:
    s, cur, count = sum(A), 0, 0
    for num in A:
      cur += num
      if cur == s / 3:
        cur = 0
        count += 1
      if count == 3:
        return True
    return False
