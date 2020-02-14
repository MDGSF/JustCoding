class Solution:
  def replaceSpaces(self, S: str, length: int) -> str:
    sList = list(S)
    for i in range(length):
      if sList[i] == ' ':
        sList[i] = '%20'
    return ''.join(sList[:length])

