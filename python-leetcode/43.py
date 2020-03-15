class Solution:
  def multiply(self, num1: str, num2: str) -> str:
    if num1 == '0' or num2 == '0':return '0'
    n1, n2 = len(num1), len(num2)
    digits = [0] * (n1 + n2)
    for i in range(n1 - 1, -1, -1):
      for j in range(n2 - 1, -1, -1):
        cur = (ord(num1[i]) - ord('0')) * (ord(num2[j]) - ord('0'))
        cur += digits[i + j + 1]
        digits[i + j] += cur // 10
        digits[i + j + 1] = cur % 10
    return ''.join([str(i) for i in digits]).lstrip('0')
