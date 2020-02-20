class Solution:
  def isMonotonic(self, A: List[int]) -> bool:
    state = 0 # 0不确定，1单调递增，2单调递减
    for i in range(1, len(A)):
      if A[i] == A[i-1]:
        continue
      elif A[i] > A[i-1]:
        if state == 0:
          state = 1
        elif state == 1:
          continue
        else:
          return False
      else:
        if state == 0:
          state = 2
        elif state == 1:
          return False
        else:
          continue
    return True
