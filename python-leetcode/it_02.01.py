# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def removeDuplicateNodes(self, head: ListNode) -> ListNode:
    s = set()
    dummy = ListNode(0)
    dummy.next = head
    pre, cur = dummy, dummy.next
    while cur != None:
      if cur.val in s:
        cur = cur.next
      else:
        s.add(cur.val)
        pre.next = cur
        pre = pre.next
        cur = cur.next
    pre.next = None
    return dummy.next

