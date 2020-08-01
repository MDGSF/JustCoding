# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def isSubStructure(self, A: TreeNode, B: TreeNode) -> bool:
    if B is None: return False
    if A is None: return False
    result = False
    if A.val == B.val:
      result = self.hasSubTree(A, B)
    if result == False:
      result = self.isSubStructure(A.left, B)
    if result == False:
      result = self.isSubStructure(A.right, B)
    return result

  def hasSubTree(self, A, B):
    if B is None: return True
    if A is None: return False
    if A.val != B.val: return False
    return self.hasSubTree(A.left, B.left) and self.hasSubTree(A.right, B.right)

