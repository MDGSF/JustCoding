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
    cur = root
    while cur:
      if cur.left:
        predecessor = cur.left
        next = cur.left
        while predecessor.right:
          predecessor = predecessor.right
        predecessor.right = cur.right
        cur.left = None
        cur.right = next
      cur = cur.right

