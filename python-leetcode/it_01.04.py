class Solution:
  def canPermutePalindrome(self, s: str) -> bool:
    m = dict(collections.Counter(s))
    count = 0
    for val in m.values():
      if val % 2 == 1:
        count += 1
    return count <= 1

