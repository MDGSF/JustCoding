# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def levelOrder(self, root: TreeNode) -> List[int]:
    if root is None: return []
    result, queue = [], [root]
    while len(queue) > 0:
      curLevelSize = len(queue)
      for _ in range(curLevelSize):
        node = queue.pop(0)
        result.append(node.val)
        if node.left: queue.append(node.left)
        if node.right: queue.append(node.right)
    return result

