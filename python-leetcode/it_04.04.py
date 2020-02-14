# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def isBalanced(self, root: TreeNode) -> bool:
    return self.dfs(root)[0]

  def dfs(self, root: TreeNode):
    if root == None: return True, 0
    lIsBalanced, lHeight = self.dfs(root.left)
    rIsBalanced, rHeight = self.dfs(root.right)
    curIsBalanced = abs(lHeight - rHeight) <= 1 and lIsBalanced and rIsBalanced
    curHeight = max(lHeight, rHeight) + 1
    return curIsBalanced, curHeight
