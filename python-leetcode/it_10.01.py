class Solution:
  def merge(self, A: List[int], m: int, B: List[int], n: int) -> None:
    """
    Do not return anything, modify A in-place instead.
    """
    i, j, k = m - 1, n - 1, m + n - 1
    while i >= 0 and j >= 0:
      if A[i] > B[j]:
        A[k] = A[i]
        k -= 1
        i -= 1
      else:
        A[k] = B[j]
        k -= 1
        j -= 1
    while j >= 0:
      A[k] = B[j]
      k -= 1
      j -= 1
