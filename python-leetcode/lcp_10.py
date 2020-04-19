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
  def minimalExecTime(self, root: TreeNode) -> float:
    # @param n: 并行数目
    # @return [0] 执行完当前节点的最小耗时
    #         [1] 当前 node 为根的时间串行之和
    def dfs(root, n):
      if root == None:
        return 0, 0
      leftMinTime, leftSumTime = dfs(root.left, n)
      rightMinTime, rightSumTime = dfs(root.right, n)
      sumTime = leftSumTime + rightSumTime
      minTime = max(leftMinTime, rightMinTime, sumTime/2) + root.val
      return minTime, sumTime + root.val
    result, _ = dfs(root, 2)
    return result

n1 = TreeNode(1)
n2 = TreeNode(3)
n3 = TreeNode(2)
n4 = TreeNode(4)
n5 = TreeNode(4)

n1.left = n2
n1.right = n3
n3.left = n4
n3.right = n5

s = Solution()
result = s.minimalExecTime(n1)
print(result)
