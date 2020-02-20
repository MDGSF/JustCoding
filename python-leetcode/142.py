# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def detectCycle(self, head: ListNode) -> ListNode:
    p1 = self.getIntersection(head)
    if p1 == None:
      return None
    p2 = head
    while p1 != p2:
      p1 = p1.next
      p2 = p2.next
    return p1

  def getIntersection(self, head):
    slow, fast = head, head
    while fast and fast.next:
      slow = slow.next
      fast = fast.next.next
      if slow == fast:
        return slow
    return None
