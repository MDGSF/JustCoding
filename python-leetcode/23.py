# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

from queue import PriorityQueue

class Solution:
  def mergeKLists(self, lists: List[ListNode]) -> ListNode:
    dummy = cur = ListNode(0)
    q = PriorityQueue()
    for ListIdx, oneList in enumerate(lists):
      if oneList:
        q.put((oneList.val, ListIdx, oneList))
    while not q.empty():
      val, ListIdx, oneList = q.get()
      cur.next = oneList
      cur = cur.next
      oneList = oneList.next
      if oneList:
        q.put((oneList.val, ListIdx, oneList))
    return dummy.next
