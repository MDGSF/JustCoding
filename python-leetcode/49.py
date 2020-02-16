class Solution:
  def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
    m = {}
    for curStr in strs:
      curKey = ''.join(sorted(curStr))
      if curKey in m:
        m[curKey].append(curStr)
      else:
        m[curKey] = [curStr]
    return [v for v in m.values()]
