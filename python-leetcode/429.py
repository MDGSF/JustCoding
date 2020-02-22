"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""
class Solution:
  def levelOrder(self, root: 'Node') -> List[List[int]]:
    if root == None: return root
    result = []
    queue = [root]
    while queue:
      curLevelSize = len(queue)
      curLevel = []
      for i in range(curLevelSize):
        node = queue.pop(0)
        curLevel.append(node.val)
        for child in node.children:
          queue.append(child)
      result.append(curLevel)
    return result
