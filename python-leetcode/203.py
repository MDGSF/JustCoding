# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def removeElements(self, head: ListNode, val: int) -> ListNode:
    dummy = ListNode(0)
    dummy.next = head
    p2, p1 = dummy, head
    while p1:
      if p1.val == val:
        p2.next = p1.next
        p1 = p1.next
      else:
        p2 = p1
        p1 = p1.next
    return dummy.next
