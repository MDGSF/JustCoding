class Solution:
  def maxDepthAfterSplit(self, seq: str) -> List[int]:
    result, depth = [], 0
    for c in seq:
      if c == '(':
        depth += 1
        result.append(depth % 2)
      if c == ')':
        result.append(depth % 2)
        depth -= 1
    return result
