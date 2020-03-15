# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def diameterOfBinaryTree(self, root: TreeNode) -> int:
    self.result = 0
    def depth(root):
      if root == None: return 0
      left = depth(root.left)
      right = depth(root.right)
      self.result = max(self.result, left + right)
      return max(left, right) + 1
    depth(root)
    return self.result
