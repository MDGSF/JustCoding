# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
  def flatten(self, root: TreeNode) -> None:
    """
    Do not return anything, modify root in-place instead.
    """
    preorderlist = []

    def preorderTraversal(root):
      if root:
        preorderlist.append(root)
        preorderTraversal(root.left)
        preorderTraversal(root.right)

    preorderTraversal(root)
    size = len(preorderlist)
    for i in range(1, size):
      pre, cur = preorderlist[i - 1], preorderlist[i]
      pre.left = None
      pre.right = cur


