# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def minDepth(self, root: TreeNode) -> int:
    if root == None: return 0
    if root.left == None and root.right == None: return 1
    if root.left != None and root.right != None:
      return min(self.minDepth(root.left), self.minDepth(root.right)) + 1
    if root.left == None:
      return self.minDepth(root.right) + 1
    else:
      return self.minDepth(root.left) + 1
