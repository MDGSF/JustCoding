from typing import List

class Solution:
  def minimumLengthEncoding(self, words: List[str]) -> int:
    self.root = {}
    self.end_of_word = '#'
    for word in words:
      self.insert(word)

    self.result, self.count = 0, 0
    def dfs(root, wordlen):
      if len(root) == 1 and self.end_of_word in root:
        self.count += 1
        self.result += wordlen
        return
      for key in root.keys():
        if key != self.end_of_word:
          dfs(root[key], wordlen + 1)
    dfs(self.root, 0)
    return self.result + self.count

  def insert(self, word):
    node = self.root
    for i in range(len(word) - 1, -1, -1):
      node = node.setdefault(word[i], {})
    node[self.end_of_word] = self.end_of_word

words = ["time", "me", "bell"]
s = Solution()
result = s.minimumLengthEncoding(words)
print(result)

