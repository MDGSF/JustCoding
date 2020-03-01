class Solution:
  def isPalindrome(self, s: str) -> bool:
    a = list(filter(lambda e: e.isdigit() or e.isalpha(), list(s.lower())))
    left, right = 0, len(a) - 1
    while left < right:
      if a[left] != a[right]:
        return False
      left += 1
      right -= 1
    return True

s = Solution()
result = s.isPalindrome("A man, a plan, a canal: Panama")
print(result)
