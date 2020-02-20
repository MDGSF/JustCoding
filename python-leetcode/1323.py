class Solution:
  def maximum69Number (self, num: int) -> int:
    a = list(str(num))
    for i in range(len(a)):
      if a[i] == '6':
        a[i] = '9'
        break
    return int(''.join(a))
