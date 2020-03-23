import collections

class Solution:
  def minWindow(self, s: str, t: str) -> str:
    m, cur = collections.Counter(t), {}
    count, left, right, minstr = len(m.keys()), 0, 0, ''
    while right < len(s) or left < len(s):
      if count == 0:
        curstr = s[left:right]
        if minstr == '' or len(curstr) < len(minstr):
          minstr = curstr
        c = s[left]
        if c in m:
          if cur[c] == m[c]:
            count += 1
          cur[c] -= 1
        left += 1
      else:
        if right >= len(s):
          break
        c = s[right]
        if c in m:
          if c in cur:
            cur[c] += 1
          else:
            cur[c] = 1
          if cur[c] == m[c]:
            count -=1
        right += 1
    return minstr



S = "ADOBECODEBANC"
T = "ABC"
s = Solution()
result = s.minWindow(S, T)
print(result)
