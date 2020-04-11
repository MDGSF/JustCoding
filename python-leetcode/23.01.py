# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def mergeKLists(self, lists: List[ListNode]) -> ListNode:
    nodes = []
    for oneList in lists:
      while oneList:
        nodes.append(oneList.val)
        oneList = oneList.next
    dummy = cur = ListNode(0)
    for val in sorted(nodes):
      newNode = ListNode(val)
      cur.next = newNode
      cur = cur.next
    return dummy.next
