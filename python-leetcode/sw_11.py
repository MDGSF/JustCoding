from typing import List

class Solution:
  def minArray(self, numbers: List[int]) -> int:
    left, right, mid = 0, len(numbers) - 1, 0
    while numbers[left] >= numbers[right]:
      if right - left == 1:
        mid = right
        break
      mid = (left + right) // 2
      if numbers[left] == numbers[mid] and numbers[mid] == numbers[right]:
        return min(numbers[left:right+1])
      elif numbers[left] <= numbers[mid]:
        left = mid
      elif numbers[mid] <= numbers[right]:
        right = mid
    return numbers[mid]

numbers = [1,0,1,1,1]
s = Solution()
result = s.minArray(numbers)
print(result)
