class Solution:
  def reverseVowels(self, s: str) -> str:
    a = list(s)
    left, right = 0, len(a) - 1
    while left < right:
      while left < len(a) and not self.isVowel(a[left]):
        left += 1
      while right >= 0 and not self.isVowel(a[right]):
        right -= 1
      if left >= right:
        break
      a[left], a[right] = a[right], a[left]
      left += 1
      right -= 1
    return ''.join(a)

  def isVowel(self, c):
    if c in {'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'}:
      return True
    return False
