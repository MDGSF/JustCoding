# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
    dummy = ListNode(0)
    cur, p1, p2 = dummy, l1, l2
    while p1 and p2:
      if p1.val < p2.val:
        cur.next = p1
        p1 = p1.next
      else:
        cur.next = p2
        p2 = p2.next
      cur = cur.next
    if p1: cur.next = p1
    if p2: cur.next = p2
    return dummy.next
