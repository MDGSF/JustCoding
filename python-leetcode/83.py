# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def deleteDuplicates(self, head: ListNode) -> ListNode:
    if head == None: return None
    p2, p1 = head, head.next
    while p1:
      if p1.val != p2.val:
        p2.next = p1
        p2 = p1
      p1 = p1.next
    p2.next = p1
    return head
