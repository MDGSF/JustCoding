# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def buildTree(self, preorder: List[int], inorder: List[int]) -> TreeNode:
    if len(preorder) == 0 or len(inorder) == 0: return None
    val = preorder[0]
    idx = inorder.index(val)
    node = TreeNode(val)
    node.left = self.buildTree(preorder[1:idx+1], inorder[:idx])
    node.right = self.buildTree(preorder[idx+1:], inorder[idx+1:])
    return node
