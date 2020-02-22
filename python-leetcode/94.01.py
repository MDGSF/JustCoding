# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def inorderTraversal(self, root: TreeNode) -> List[int]:
    result = []
    stack = []
    node = root
    while node != None or len(stack) > 0:
      while node != None:
        stack.append(node)
        node = node.left
      node = stack.pop()
      result.append(node.val)
      node = node.right
    return result
