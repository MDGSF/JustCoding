class Solution:

  def largestTriangleArea(self, points: List[List[int]]) -> float:
    result = 0
    for i in range(len(points)):
      for j in range(i + 1, len(points)):
        for k in range(j + 1, len(points)):
          curResult = self.area(points[i], points[j], points[k])
          result = max(result, curResult)
    return result

  def area(self, p, q, r):
    return .5 * abs(p[0] * q[1] + q[0] * r[1] + r[0] * p[1] - p[1] * q[0] -
                    q[1] * r[0] - r[1] * p[0])
