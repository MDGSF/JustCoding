# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
    s1, s2 = [], []
    while l1:
      s1.append(l1.val)
      l1 = l1.next
    while l2:
      s2.append(l2.val)
      l2 = l2.next
    l3 = None
    carry = 0
    while s1 or s2 or carry != 0:
      a = s1.pop() if s1 else 0
      b = s2.pop() if s2 else 0
      cur = a + b + carry
      carry = cur // 10
      cur = cur % 10
      curNode = ListNode(cur)
      curNode.next = l3
      l3 = curNode
    return l3
