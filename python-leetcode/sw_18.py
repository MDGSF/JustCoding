# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def deleteNode(self, head: ListNode, val: int) -> ListNode:
    if head.val == val:
      return head.next
    cur = head
    while cur and cur.next:
      if cur.next.val == val:
        cur.next = cur.next.next
        break
      cur = cur.next
    return head
