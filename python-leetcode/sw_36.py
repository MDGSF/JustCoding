"""
# Definition for a Node.
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
"""
class Solution:
  def treeToDoublyList(self, root: 'Node') -> 'Node':
    def dfs(cur):
      if not cur: return
      dfs(cur.left)
      if self.pre:
        self.pre.right, cur.left = cur, self.pre
      else:
        self.head = cur
      self.pre = cur
      dfs(cur.right)
    if not root: return
    self.head = self.pre = None
    dfs(root)
    self.head.left, self.pre.right = self.pre, self.head
    return self.head

