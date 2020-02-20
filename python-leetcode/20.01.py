class Solution:
  def isValid(self, s: str) -> bool:
    m = {'(':')', '[':']', '{':'}', '?':'?'}
    stack = ['?']
    for c in s:
      if c in m: stack.append(c)
      elif c != m[stack.pop()]: return False
    return len(stack) == 1
