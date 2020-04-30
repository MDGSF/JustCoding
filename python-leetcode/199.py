# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def rightSideView(self, root: TreeNode) -> List[int]:
    result = []
    def dfs(root, level):
      if root == None: return
      if level + 1 > len(result):
        result.append(root.val)
      dfs(root.right, level + 1)
      dfs(root.left, level + 1)
    dfs(root, 0)
    return result
