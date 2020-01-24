# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def swapPairs(self, head: ListNode) -> ListNode:
    dummy = ListNode()
    dummy.next = head
    pre, cur = dummy, head
    while cur != None and cur.next != None:
      post = cur.next
      cur.next = post.next
      post.next = cur
      pre.next = post
      pre = cur
      cur = cur.next
    return dummy.next

