class Solution:
  def reverseOnlyLetters(self, S: str) -> str:
    a = list(S)
    i, j = 0, len(S) - 1
    while True:
      while i < j and not a[i].isalpha():
        i += 1
      while i < j and not a[j].isalpha():
        j -= 1
      if i >= j:
        break
      a[i], a[j] = a[j], a[i]
      i += 1
      j -= 1
    return ''.join(a)


s = Solution()
result = s.reverseOnlyLetters("ab-cd")
print(result)
