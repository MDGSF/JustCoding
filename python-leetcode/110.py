# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def isBalanced(self, root: TreeNode) -> bool:
    def dfs(node):
      if node == None: return 0, True
      lHeight, lIsBalanced = dfs(node.left)
      rHeight, rIsBalanced = dfs(node.right)
      return max(lHeight, rHeight) + 1, \
        abs(lHeight - rHeight) <= 1 and lIsBalanced and rIsBalanced
    return dfs(root)[1]
