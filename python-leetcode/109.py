# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
  def sortedListToBST(self, head: ListNode) -> TreeNode:
    size = self.listlen(head)
    self.curHead = head
    return self.dfs(0, size - 1)

  def dfs(self, start, end):
    if start > end: return None
    mid = (start + end) // 2
    left = self.dfs(start, mid - 1)
    node = TreeNode(self.curHead.val)
    node.left = left
    self.curHead = self.curHead.next
    node.right = self.dfs(mid + 1, end)
    return node

  def listlen(self, head):
    cur = head
    count = 0
    while cur:
      count += 1
      cur = cur.next
    return count
