class Solution:
  def canThreePartsEqualSum(self, A: List[int]) -> bool:
    s = sum(A)
    if s % 3 != 0:
      return False
    target = s // 3
    n, i, cur = len(A), 0, 0
    while i < n:
      cur += A[i]
      if cur == target:
        break
      i += 1
    if cur != target:
      return False
    i, cur = i + 1, 0
    while i < n:
      cur += A[i]
      if cur == target:
        break
      i += 1
    if cur != target:
      return False
    return True if i < n - 1 else False
