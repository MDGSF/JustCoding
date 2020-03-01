import collections

class Solution:
  def firstUniqChar(self, s: str) -> int:
    counter = collections.Counter(s)
    for i, c in enumerate(s):
      if counter[c] == 1:
        return i
    return -1
