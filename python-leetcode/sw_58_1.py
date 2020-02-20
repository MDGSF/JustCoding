class Solution:
  def reverseWords(self, s: str) -> str:
    return ' '.join(list(reversed(s.strip().split())))
