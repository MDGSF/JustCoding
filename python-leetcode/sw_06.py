# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
  def reversePrint(self, head: ListNode) -> List[int]:
    result, cur = [], head
    while cur:
      result.append(cur.val)
      cur = cur.next
    result.reverse()
    return result
