class Solution:
  def verifyPostorder(self, postorder: List[int]) -> bool:
    def recursion(startIdx, endIdx):
      if startIdx >= endIdx: return True
      curIdx = startIdx
      while postorder[curIdx] < postorder[endIdx]:
        curIdx += 1
      rightStartIdx = curIdx
      while postorder[curIdx] > postorder[endIdx]:
        curIdx += 1
      return curIdx == endIdx and \
          recursion(startIdx, rightStartIdx - 1) and \
          recursion(rightStartIdx, endIdx - 1)
    return recursion(0, len(postorder) - 1)

