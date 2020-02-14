class Solution:
  def compressString(self, S: str) -> str:
    news = ''.join(key + str(len(list(group))) for key, group in itertools.groupby(S))
    return len(news) < len(S) and news or S

