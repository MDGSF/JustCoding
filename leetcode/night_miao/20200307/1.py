import collections

class Solution:
  def sortString(self, s: str) -> str:
    counter, result, flag = collections.Counter(s), [], True
    while len(counter) > 0:
      if flag: start, end, step = ord('a'), ord('z') + 1, 1
      else: start, end, step = ord('z'), ord('a') - 1, -1
      for i in range(start, end, step):
        c = chr(i)
        if c in counter:
          result.append(c)
          if counter[c] == 1: del counter[c]
          else: counter[c] -= 1
      flag = not flag
    return ''.join(result)

s = "aaaabbbbcccc"
# s = "rat"
# s = "leetcode"
solu = Solution()
result = solu.sortString(s)
print(result)

