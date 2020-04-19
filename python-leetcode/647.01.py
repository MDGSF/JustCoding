class Solution:
  def countSubstrings(self, s: str) -> int:
    self.count = 0
    def extendPalindrome(left, right):
      while left >= 0 and right < len(s) and s[left] == s[right]:
        self.count += 1
        left -= 1
        right += 1
    for i in range(len(s)): # i is the mid point
      extendPalindrome(i, i) # odd length
      extendPalindrome(i, i + 1) # even length
    return self.count

s = "abc"
solu = Solution()
result = solu.countSubstrings(s)
print(result)
