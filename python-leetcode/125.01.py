class Solution:
  def isPalindrome(self, s: str) -> bool:
    a = [*filter(str.isalnum, s.lower())]
    return a == a[::-1]

s = Solution()
result = s.isPalindrome("A man, a plan, a canal: Panama")
print(result)
