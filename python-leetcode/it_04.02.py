# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def sortedArrayToBST(self, nums: List[int]) -> TreeNode:
    if len(nums) == 0:
      return None
    mid = len(nums) // 2
    midVal = nums[mid]
    node = TreeNode(midVal)
    node.left = self.sortedArrayToBST(nums[:mid])
    node.right = self.sortedArrayToBST(nums[mid+1:])
    return node
