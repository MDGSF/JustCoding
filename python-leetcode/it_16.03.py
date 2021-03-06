class Solution:

  def intersection(self, start1: List[int], end1: List[int], start2: List[int],
                   end2: List[int]) -> List[float]:
    x1, y1 = start1
    x2, y2 = end1
    x3, y3 = start2
    x4, y4 = end2
    result = []
    # 判断直线 (x1, y1)~(x2, y2) 和 (x3, y3)~(x4, y4) 是否平行
    if self.isParallel(x1, y1, x2, y2, x3, y3, x4, y4):
      # 平行，判断 (x3, y3) 是否在直线 (x1, y1)~(x2, y2) 上面。
      if (y2 - y1) * (x3 - x1) == (y3 - y1) * (x2 - x1):
        # 判断 (x3, y3) 是否在线段 (x1, y1)~(x2, y2) 上面
        if self.inside(x1, y1, x2, y2, x3, y3):
          result = self.update(result, x3, y3)
        # 判断 (x4, y4) 是否在线段 (x1, y1)~(x2, y2) 上面
        if self.inside(x1, y1, x2, y2, x4, y4):
          result = self.update(result, x4, y4)
        # 判断 (x1, y1) 是否在线段 (x3, y3)~(x4, y4) 上面
        if self.inside(x3, y3, x4, y4, x1, y1):
          result = self.update(result, x1, y1)
        # 判断 (x2, y2) 是否在线段 (x3, y3)~(x4, y4) 上面
        if self.inside(x3, y3, x4, y4, x2, y2):
          result = self.update(result, x2, y2)
      # 平行时，其它情况都不会有交点
    else:
      t1 = (x3 * (y4 - y3) + y1 * (x4 - x3) - y3 * (x4 - x3) - x1 *
            (y4 - y3)) / ((x2 - x1) * (y4 - y3) - (x4 - x3) * (y2 - y1))
      t2 = (x1 * (y2 - y1) + y3 * (x2 - x1) - y1 * (x2 - x1) - x3 *
            (y2 - y1)) / ((x4 - x3) * (y2 - y1) - (x2 - x1) * (y4 - y3))
      if 0.0 <= t1 <= 1.0 and 0.0 <= t2 <= 1.0:
        result = [x1 + t1 * (x2 - x1), y1 + t1 * (y2 - y1)]
    return result

  # 判断两条直线是否平行
  def isParallel(self, x1, y1, x2, y2, x3, y3, x4, y4):
    return (x4 - x3) * (y2 - y1) == (x2 - x1) * (y4 - y3)

  # 判断 (xk, yk) 是否在线段 (x1, y1)~(x2, y2) 上面
  # 前提是 (xk, yk) 必须在直线 (x1, y1)~(x2, y2) 上面
  # 若与 x 轴平行，只需要判断 x 的部分，(y1 == y2)
  # 若与 y 轴平行，只需要判断 y 的部分，(x1 == x2)
  # 若为普通线段，则都要判断
  def inside(self, x1, y1, x2, y2, xk, yk):
    return (x1 == x2 or min(x1, x2) <= xk <= max(x1, x2)) and \
      (y1 == y2 or min(y1, y2) <= yk <= max(y1, y2))

  def update(self, result, xk, yk):
    return [xk, yk] if not result or [xk, yk] < result else result
