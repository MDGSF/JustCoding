"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""
class Solution:
  def postorder(self, root: 'Node') -> List[int]:
    result = []
    def dfs(root):
      if root == None: return
      for child in root.children:
        dfs(child)
      result.append(root.val)
    dfs(root)
    return result
