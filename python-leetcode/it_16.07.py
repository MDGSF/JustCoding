class Solution:
  def maximum(self, a: int, b: int) -> int:
    return int((a + b) / 2 + abs((a - b)) / 2)
