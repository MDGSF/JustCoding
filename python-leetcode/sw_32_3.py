# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def levelOrder(self, root: TreeNode) -> List[List[int]]:
    if root == None: return []
    result, queue, level = [], [root], 0
    while len(queue) > 0:
      curLevelSize = len(queue)

      curLevel = []
      if level % 2 == 0:
        startIndex, endIndex, step = 0, curLevelSize, 1
      else:
        startIndex, endIndex, step = curLevelSize - 1, -1, -1
      for i in range(startIndex, endIndex, step):
        curLevel.append(queue[i].val)
      result.append(curLevel)

      for _ in range(curLevelSize):
        node = queue.pop(0)
        if node.left: queue.append(node.left)
        if node.right: queue.append(node.right)

      level += 1
    return result

