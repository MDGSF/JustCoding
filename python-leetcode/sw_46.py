class Solution:
  def translateNum(self, num: int) -> int:
    s = str(num)
    f0, f1 = 1, 1
    for i in range(1, len(s)):
      f0, f1 = f1, (f0 + f1 if "10" <= s[i-1:i+1] <= "25" else f1)
    return f1;

num = 12258

s = Solution()
result = s.translateNum(num)
print(result)

