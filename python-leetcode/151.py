class Solution:
  def reverseWords(self, s: str) -> str:
    return ' '.join(list(reversed(s.strip().split())))

s = Solution()
result = s.reverseWords("the sky is blue")
print(result)
