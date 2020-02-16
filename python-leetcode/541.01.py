class Solution:
  def reverseStr(self, s: str, k: int) -> str:
    sList = list(s)
    i, n = 0, len(s)
    while i < n:
      if n - i < k:
        self.reverse(sList, i, n - 1)
        break
      elif k <= n - i < 2*k:
        self.reverse(sList, i, i + k - 1)
        break
      else:
        self.reverse(sList, i, i + k - 1)
        i += 2 * k
    return ''.join(sList)

  def reverse(self, s, start, end):
    while start < end:
      s[start], s[end] = s[end], s[start]
      start += 1
      end -= 1
