class Solution:
  def numJewelsInStones(self, J: str, S: str) -> int:
    Jset = set(J)
    return sum(c in Jset for c in S)

