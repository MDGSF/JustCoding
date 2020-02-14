from typing import List


class Solution:

  def hanota(self, A: List[int], B: List[int], C: List[int]) -> None:
    """
    Do not return anything, modify C in-place instead.
    """

    def dfs(n, A, B, C):
      if n == 0:
        return
      if n == 1:
        C.append(A.pop())
        return
      dfs(n - 1, A, C, B)
      C.append(A.pop())
      dfs(n - 1, B, A, C)

    dfs(len(A), A, B, C)


A = [2, 1, 0]
B = []
C = []
s = Solution()
s.hanota(A, B, C)
print(A, B, C)
