import collections

class Solution:
  def longestPalindrome(self, s: str) -> int:
    result = 0
    counter = collections.Counter(s)
    for c in counter.values():
      if c >= 0:
        if c % 2 == 0:
          result += c
        else:
          result += c - 1
    return result + 1 if result < len(s) else result
