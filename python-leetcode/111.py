# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def minDepth(self, root: TreeNode) -> int:
    if root == None: return 0
    left = self.minDepth(root.left)
    right = self.minDepth(root.right)
    return left + right + 1 if left == 0 or right == 0 else min(left, right) + 1
