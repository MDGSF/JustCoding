from typing import List

class Solution:
  def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
    wordListSet = set(wordList)
    if endWord not in wordListSet: return 0
    result, visited, s1, s2 = 1, set(), {beginWord}, {endWord}
    while len(s1) > 0 and len(s2) > 0:
      if len(s1) > len(s2): s1, s2, = s2, s1
      result += 1
      newS = set()
      for word in s1:
        visited.add(word)
        for i in range(len(word)):
          for c in range(ord('a'), ord('z')  + 1):
            newWord = word[:i] + chr(c) + word[i+1:]
            if newWord in s2:
              return result
            if newWord not in visited and newWord in wordListSet:
              newS.add(newWord)
      s1 = newS
    return 0



beginWord = "hit"
endWord = "cog"
wordList = ["hot","dot","dog","lot","log","cog"]

# beginWord = "hit"
# endWord = "cog"
# wordList = ["hot","dot","dog","lot","log"]

s = Solution()
result = s.ladderLength(beginWord, endWord, wordList)
print(result)
