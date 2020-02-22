# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def inorderTraversal(self, root: TreeNode) -> List[int]:
    result = []
    def dfs(root):
      if root == None: return
      dfs(root.left)
      result.append(root.val)
      dfs(root.right)
    dfs(root)
    return result
