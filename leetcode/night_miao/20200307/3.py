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

  def longestZigZag(self, root: TreeNode) -> int:
    self.result = 0
    self.dfs(root)
    return self.result

  def dfs(self, root):
    if root == None: return 0, 0
    if root.left == None and root.right == None: return 0, 0

    rootL, rootR = 0, 0
    if root.left == None:
      rootL = 0
    else:
      l, r = self.dfs(root.left)
      rootL = r + 1

    if root.right == None:
      rootR = 0
    else:
      l, r = self.dfs(root.right)
      rootR = l + 1

    self.result = max(self.result, rootL, rootR)
    return rootL, rootR

node1 = TreeNode(1)
node2 = TreeNode(2)
node3 = TreeNode(3)
node4 = TreeNode(4)
node5 = TreeNode(5)
node6 = TreeNode(6)
node7 = TreeNode(7)
node8 = TreeNode(8)

node1.right = node2
node2.left = node3
node2.right = node4
node4.left = node5
node4.right = node6
node5.right = node7
node7.right = node8

s = Solution()
result = s.longestZigZag(node1)
print(result)
