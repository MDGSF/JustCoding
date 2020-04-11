# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def balanceBST(self, root: TreeNode) -> TreeNode:
    nodes = []
    def inorder(root):
      if root == None: return
      inorder(root.left)
      nodes.append(root)
      inorder(root.right)
    inorder(root)

    def buildTree(start, end):
      if start > end: return None
      mid = start + (end - start) // 2
      node = TreeNode(nodes[mid].val)
      node.left = buildTree(start, mid - 1)
      node.right = buildTree(mid + 1, end)
      return node
    return buildTree(0, len(nodes) - 1)
