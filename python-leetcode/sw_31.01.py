from typing import List

class Solution:
  def validateStackSequences(self, pushed: List[int], popped: List[int]) -> bool:
    stack = []
    for num in popped:
      if len(stack) == 0 or stack[-1] != num:
        while len(pushed) > 0 and pushed[0] != num:
          stack.append(pushed[0])
          pushed.pop(0)
        if len(pushed) == 0:
          return False
        else:
          pushed.pop(0)
      else:
        stack.pop()
    return True

pushed = [1,2,3,4,5]
popped = [4,5,3,2,1]

pushed = [1,2,3,4,5]
popped = [4,3,5,1,2]

s = Solution()
result = s.validateStackSequences(pushed, popped)
print(result)

