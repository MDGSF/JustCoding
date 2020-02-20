# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def reverseList(self, head: ListNode) -> ListNode:
    pre, cur = None, head
    while cur != None:
      next = cur.next
      cur.next = pre
      pre = cur
      cur = next
    return pre
