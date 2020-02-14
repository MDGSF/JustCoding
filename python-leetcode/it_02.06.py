# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def isPalindrome(self, head: ListNode) -> bool:
    stack = []
    cur = head
    while cur != None:
      stack.append(cur.val)
      cur = cur.next

    cur = head
    while cur != None:
      if cur.val != stack.pop():
        return False
      cur = cur.next
    return True

