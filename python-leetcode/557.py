class Solution:
  def reverseWords(self, s: str) -> str:
    return ' '.join([word[::-1] for word in s.split(" ")])

s = Solution()
result = s.reverseWords("Let's take LeetCode contest")
print(result)
