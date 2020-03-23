from typing import List
import collections

class Solution:
  def countCharacters(self, words: List[str], chars: str) -> int:
    result, charsCounter = 0, collections.Counter(chars)
    for word in words:
      wordCounter = collections.Counter(word)
      isValidWord = True
      for c in wordCounter:
        if c not in charsCounter or wordCounter[c] > charsCounter[c]:
          isValidWord = False
          break
      if isValidWord:
        result += len(word)
    return result

words = ["cat","bt","hat","tree"]
chars = "atach"
s = Solution()
result = s.countCharacters(words, chars)
print(result)
