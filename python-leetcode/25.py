# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def reverseKGroup(self, head: ListNode, k: int) -> ListNode:
    dummy = ListNode(0)
    dummy.next = head
    pre, end = dummy, dummy
    while end.next != None:
      for i in range(k):
        if end != None:
          end = end.next
      if end == None:
        break

      start = pre.next
      next = end.next
      end.next = None
      pre.next = self.reverse(start)
      start.next = next

      pre = start
      end = pre
    return dummy.next

  def reverse(self, head):
    pre, cur = None, head
    while cur != None:
      post = cur.next
      cur.next = pre
      pre = cur
      cur = post
    return pre

