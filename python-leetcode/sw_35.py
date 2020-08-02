"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""
class Solution:
  def copyRandomList(self, head: 'Node') -> 'Node':
    self.cloneNodes(head)
    self.connectSiblingNodes(head)
    return self.reconnectNodes(head)

  def cloneNodes(self, head):
    node = head
    while node:
      newNode = Node(node.val, node.next)
      node.next = newNode
      node = newNode.next

  def connectSiblingNodes(self, head):
    node = head
    while node:
      cloned = node.next
      if node.random:
        cloned.random = node.random.next
      node = cloned.next

  def reconnectNodes(self, head):
    node = head
    clonedHead = None
    clonedNode = None
    if node:
      clonedHead = node.next
      clonedNode = node.next
      node.next = clonedNode.next
      node = node.next
    while node:
      clonedNode.next = node.next
      clonedNode = clonedNode.next
      node.next = clonedNode.next
      node = node.next
    return clonedHead


