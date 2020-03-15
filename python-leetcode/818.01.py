import collections

# 暴力 bfs 会超时
class Solution:
  def racecar(self, target: int) -> int:
    result = 0
    queue = collections.deque()
    queue.append((0, 1)) # position: 0, speed: 1
    while queue:
      curLevelSize = len(queue)
      for _ in range(curLevelSize):
        node = queue.popleft()
        curPosition, curSpeed = node[0], node[1]
        if curPosition == target:
          return result

        # process A
        newPosition = curPosition + curSpeed
        newSpeed = curSpeed * 2
        queue.append((newPosition, newSpeed))

        # process R
        newPosition = curPosition
        newSpeed = -1 if curSpeed > 0 else 1
        queue.append((newPosition, newSpeed))
      result += 1
    return result

target = 3 # 2
target = 6 # 5
target = 5 # 7

s = Solution()
result = s.racecar(5)
print(result)
