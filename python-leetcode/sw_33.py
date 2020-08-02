from typing import List

class Solution:
  def verifyPostorder(self, postorder: List[int]) -> bool:
    if len(postorder) <= 1: return True
    root = postorder[-1]
    curIdx = 0
    while postorder[curIdx] < root:
      curIdx += 1
    rightStartIdx = curIdx
    while curIdx < len(postorder) - 1:
      if postorder[curIdx] < root:
        return False
      curIdx += 1
    return self.verifyPostorder(postorder[:rightStartIdx]) and \
        self.verifyPostorder(postorder[rightStartIdx:len(postorder)-1])


postorder = [4, 8, 6, 12, 16, 14, 10]
s = Solution()
result = s.verifyPostorder(postorder)
print(result)

