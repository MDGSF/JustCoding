# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def getKthFromEnd(self, head: ListNode, k: int) -> ListNode:
    p1 = head
    for _ in range(k):
      if p1:
        p1 = p1.next
      else:
        return None
    p2 = head
    while p1:
      p1 = p1.next
      p2 = p2.next
    return p2
