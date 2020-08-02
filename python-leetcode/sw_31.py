from typing import List

class Solution:
  def validateStackSequences(self, pushed: List[int], popped: List[int]) -> bool:
    stack, i = [], 0
    for num in pushed:
      stack.append(num)
      while len(stack) > 0 and stack[-1] == popped[i]:
        stack.pop()
        i += 1
    return len(stack) == 0


pushed = [1,2,3,4,5]
popped = [4,3,5,1,2]


pushed = [1,2,3,4,5]
popped = [4,5,3,2,1]

s = Solution()
result = s.validateStackSequences(pushed, popped)
print(result)

