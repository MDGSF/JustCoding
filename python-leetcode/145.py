# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def postorderTraversal(self, root: TreeNode) -> List[int]:
    result = []
    def dfs(root):
      if root == None: return
      dfs(root.left)
      dfs(root.right)
      result.append(root.val)
    dfs(root)
    return result
