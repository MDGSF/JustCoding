# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def binaryTreePaths(self, root: TreeNode) -> List[str]:
    result = []
    def dfs(root, node):
      if root == None: return
      if root.left == None and root.right == None:
        result.append(node + [str(root.val)])
        return
      dfs(root.left, node + [str(root.val)])
      dfs(root.right, node + [str(root.val)])
    dfs(root, [])
    return map(lambda e: '->'.join(e), result)
