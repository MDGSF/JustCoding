class Solution:
  def reverseLeftWords(self, s: str, n: int) -> str:
    n = n % len(s)
    a = list(s)
    self.reverse(a, 0, n - 1)
    self.reverse(a, n, len(a) - 1)
    self.reverse(a, 0, len(a) - 1)
    return ''.join(a)

  def reverse(self, a, left, right):
    while left < right:
      a[left], a[right] = a[right], a[left]
      left += 1
      right -= 1
