# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def kthLargest(self, root: TreeNode, k: int) -> int:
    traversePath = []
    def inorderReverse(root):
      if root:
        inorderReverse(root.right)
        if len(traversePath) == k: return traversePath[-1]

        traversePath.append(root.val)
        if len(traversePath) == k: return traversePath[-1]

        inorderReverse(root.left)
        if len(traversePath) == k: return traversePath[-1]
    return inorderReverse(root)
