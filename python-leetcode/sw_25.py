# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
    dummy = ListNode(0)
    cur = dummy
    while l1 and l2:
      if l1.val < l2.val:
        cur.next = l1
        cur = cur.next
        l1 = l1.next
      else:
        cur.next = l2
        cur = cur.next
        l2 = l2.next
    if l1: cur.next = l1
    if l2: cur.next = l2
    return dummy.next
