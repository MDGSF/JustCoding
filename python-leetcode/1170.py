class Solution:
  def numSmallerByFrequency(self, queries: List[str], words: List[str]) -> List[int]:
    queriesCount = self.calcArrayCount(queries)
    wordsCount = self.calcArrayCount(words)
    result = []
    for i in range(len(queriesCount)):
      curResult = 0
      for j in range(len(wordsCount)):
        if queriesCount[i] < wordsCount[j]:
          curResult += 1
      result.append(curResult)
    return result

  def calcArrayCount(self, a):
    return [self.calcCount(s) for s in a]

  def calcCount(self, s):
    ans = [0] * 26
    for c in s:
      ans[ord(c) - ord('a')] += 1
    for i in range(len(ans)):
      if ans[i] > 0:
        return ans[i]
