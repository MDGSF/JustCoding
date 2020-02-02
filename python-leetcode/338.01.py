class Solution:
  def countBits(self, num: int) -> List[int]:
    result = []
    for i in range(num + 1):
      result.append(self.popcount(i))
    return result

  def popcount(self, x: int) -> int:
    count = 0
    while x != 0:
      x = x & (x - 1)
      count += 1
    return count

