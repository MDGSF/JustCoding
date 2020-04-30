# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def mergeKLists(self, lists: List[ListNode]) -> ListNode:
    dummy = ListNode(0)
    cur = dummy
    while True:
      NoneCount = 0
      minIdx, minVal = -1, float('inf')
      for i in range(len(lists)):
        if lists[i] == None:
          NoneCount += 1
          continue
        if lists[i].val < minVal:
          minIdx, minVal = i, lists[i].val
      if NoneCount == len(lists):
        break
      cur.next = lists[minIdx]
      cur = cur.next
      lists[minIdx] = lists[minIdx].next
      if lists[minIdx] == None:
        lists.pop(minIdx)
    return dummy.next
