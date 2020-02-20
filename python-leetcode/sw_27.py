# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def mirrorTree(self, root: TreeNode) -> TreeNode:
    if root == None: return None
    left = self.mirrorTree(root.left)
    right = self.mirrorTree(root.right)
    root.right = left
    root.left = right
    return root
