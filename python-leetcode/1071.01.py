import math
class Solution:
  def gcdOfStrings(self, str1: str, str2: str) -> str:
    candidate_len = math.gcd(len(str1), len(str2))
    candidate = str1[:candidate_len]
    if candidate * (len(str1) // candidate_len) == str1 and \
      candidate * (len(str2) // candidate_len) == str2:
      return candidate
    return ''
