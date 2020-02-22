class TreeNode:

  def __init__(self, x):
    self.val = x
    self.left = None
    self.right = None


# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:

  def isValidBST(self, root: TreeNode) -> bool:

    def dfs(root, minval, maxval):
      if root == None: return True
      if minval != None and root.val <= minval: return False
      if maxval != None and root.val >= maxval: return False
      return dfs(root.left, minval, root.val) and \
        dfs(root.right, root.val, maxval)

    return dfs(root, None, None)

node1 = TreeNode(0)
node2 = TreeNode(-1)
node1.right = node2
root = node1

s = Solution()
result = s.isValidBST(root)
print(result)
