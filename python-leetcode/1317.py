class Solution:
  def getNoZeroIntegers(self, n: int) -> List[int]:
    for A in range(10**4):
      B = n - A
      if '0' not in str(A) + str(B):
        return [A, B]
    return []
