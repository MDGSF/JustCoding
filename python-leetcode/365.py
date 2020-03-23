import collections
class Solution:
  def canMeasureWater(self, x: int, y: int, z: int) -> bool:
    a, b = 0, 0
    queue = collections.deque()
    queue.append((a, b))
    visited = set((a, b))
    while queue:
      node = queue.popleft()
      a, b = node[0], node[1]
      if a + b == z:
        return True

      # a = 0
      if a > 0 and (0, b) not in visited:
        queue.append((0, b))
        visited.add((0, b))

      # a = y
      if a < x and (x, b) not in visited:
        queue.append((x, b))
        visited.add((x, b))

      # b = 0
      if b > 0 and (a, 0) not in visited:
        queue.append((a, 0))
        visited.add((a, 0))

      # b = y
      if b < y and (a, y) not in visited:
        queue.append((a, y))
        visited.add((a, y))

      # a -> b
      if a > 0 and b < y:
        t = min(a, y - b)
        if (a - t, b + t) not in visited:
          queue.append((a - t, b + t))
          visited.add((a - t, b + t))

      # b -> a
      if b > 0 and a < x:
        t = min(x - a, b)
        if (a + t, b - t) not in visited:
          queue.append((a + t, b - t))
          visited.add((a + t, b - t))

    return False

s = Solution()
result = s.canMeasureWater(2, 6, 5)
print(result)
