class Solution:
  def isRobotBounded(self, instructions: str) -> bool:
    d = 0
    dx = [0, 1, 0, -1] #上、右、下、左
    dy = [1, 0, -1, 0] #上、右、下、左
    x, y = 0, 0
    for c in instructions:
      if c == 'R':
        d += 1
      elif c == 'L':
        d += 3
      elif c == 'G':
        d = d % 4
        x = x + dx[d]
        y = y + dy[d]
    return (x == 0 and y == 0) or (d % 4 != 0)
