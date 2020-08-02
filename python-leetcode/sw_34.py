# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

import copy

class Solution:
  def pathSum(self, root: TreeNode, sum: int) -> List[List[int]]:
    if root is None: return []
    result = []
    def dfs(node, path, curSum):
      path.append(node.val)
      curSum += node.val
      isLeaf = node.left == None and node.right == None
      if isLeaf and curSum == sum:
        result.append(copy.deepcopy(path))
        path.pop()
        return
      if node.left: dfs(node.left, path, curSum)
      if node.right: dfs(node.right, path, curSum)
      path.pop()
    dfs(root, [], 0)
    return result

