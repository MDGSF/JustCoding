class Solution:
  def reverseOnlyLetters(self, S: str) -> str:
    a, i, j = list(S), 0, len(S) - 1
    while i < j:
      if not a[i].isalpha():
        i += 1
      elif not a[j].isalpha():
        j -= 1
      else:
        a[i], a[j] = a[j], a[i]
        i += 1
        j -= 1
    return ''.join(a)
