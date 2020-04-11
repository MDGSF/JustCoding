class Solution:
  def maxDepthAfterSplit(self, seq: str) -> List[int]:
    result, a, b = [], 0, 0
    for c in seq:
      if c == '(':
        if a <= b:
          a += 1
          result.append(0)
        else:
          b += 1
          result.append(1)
      elif c == ')':
        if a > b:
          a -= 1
          result.append(0)
        else:
          b -= 1
          result.append(1)
    return result
