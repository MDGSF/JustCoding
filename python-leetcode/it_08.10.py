import collections
from typing import List


class Solution:

  def floodFill(self, image: List[List[int]], sr: int, sc: int,
                newColor: int) -> List[List[int]]:
    if image[sr][sc] == newColor: return image
    oldColor = image[sr][sc]
    image[sr][sc] = newColor
    queue = collections.deque()
    queue.append([sr, sc])
    while len(queue) > 0:
      cur = queue.popleft()
      curRow, curCol = cur[0], cur[1]
      for oneDir in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
        newRow, newCol = curRow + oneDir[0], curCol + oneDir[1]
        if 0 <= newRow < len(image) and \
          0 <= newCol < len(image[0]) and \
          image[newRow][newCol] == oldColor:
          image[newRow][newCol] = newColor
          queue.append([newRow, newCol])
    return image

image = [[0,0,0],[0,1,1]]
sr = 1
sc = 1
newColor = 1
# image = [[1,1,1],[1,1,0],[1,0,1]]
# sr = 1
# sc = 1
# newColor = 2
s = Solution()
result = s.floodFill(image, sr, sc, newColor)
print(result)
