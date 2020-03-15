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
  def maxSumBST(self, root: TreeNode) -> int:
    self.result = 0
    self.dfs(root)
    return 0 if self.result < 0 else self.result

  def dfs(self, root):
    if root == None: return True, 0
    if root.left == None and root.right == None:
      self.result = max(self.result, root.val)
      return True, root.val
    lIsBST, lSum = self.dfs(root.left)
    rIsBST, rSum = self.dfs(root.right)
    rootIsBST, rootSum = False, root.val
    if not lIsBST or not rIsBST or \
      (root.left and root.left.val >= root.val) or \
      (root.right and root.right.val <= root.val):
      rootIsBST = False
    else:
      rootIsBST = True
      rootSum = rootSum + lSum + rSum
      self.result = max(self.result, rootSum)
    return rootIsBST, rootSum

node1 = TreeNode(1)
node2 = TreeNode(4)
node3 = TreeNode(3)
node4 = TreeNode(2)
node5 = TreeNode(4)
node6 = TreeNode(2)
node7 = TreeNode(5)
node8 = TreeNode(4)
node9 = TreeNode(6)

node1.left = node2
node1.right = node3
node2.left = node4
node2.right = node5
node3.left = node6
node3.right = node7
node7.left = node8
node7.right = node9


# node1 = TreeNode(4)
# node2 = TreeNode(3)
# node3 = TreeNode(1)
# node4 = TreeNode(2)

# node1.left = node2
# node2.left = node3
# node2.right = node4


# node1 = TreeNode(5)
# node2 = TreeNode(4)
# node3 = TreeNode(8)
# node4 = TreeNode(3)
# node5 = TreeNode(6)
# node6 = TreeNode(3)

# node1.left = node2
# node1.right = node3
# node2.left = node4
# node3.left = node5
# node3.right = node6

s = Solution()
result = s.maxSumBST(node1)
print(result)
