# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  # 从右向左的中序遍历
  def bstToGst(self, root: TreeNode) -> TreeNode:
    self.cur = 0
    def dfs(root):
      if root == None: return
      dfs(root.right)
      self.cur += root.val
      root.val = self.cur
      dfs(root.left)
    dfs(root)
    return root
