import collections

class Twitter:

  def __init__(self):
    """
    Initialize your data structure here.
    """
    self.time = 0
    self.posts = collections.defaultdict(list)
    self.followees = collections.defaultdict(set)

  def postTweet(self, userId: int, tweetId: int) -> None:
    """
    Compose a new tweet.
    """
    self.posts[userId].append([self.time, tweetId])
    self.time += 1

  def getNewsFeed(self, userId: int) -> List[int]:
    """
    Retrieve the 10 most recent tweet ids in the user's news feed.
    Each item in the news feed must be posted by users who the user followed
    or by the user herself. Tweets must be ordered from most recent to least recent.
    """
    t = []
    t.extend(self.posts[userId][-10:])
    for followee in self.followees[userId]:
      t.extend(self.posts[followee][-10:])
    t.sort(key = lambda x: -x[0])
    result = []
    for e in t[:10]:
      result.append(e[1])
    return result

  def follow(self, followerId: int, followeeId: int) -> None:
    """
    Follower follows a followee. If the operation is invalid, it should be a no-op.
    """
    if followerId != followeeId:
      self.followees[followerId].add(followeeId)

  def unfollow(self, followerId: int, followeeId: int) -> None:
    """
    Follower unfollows a followee. If the operation is invalid, it should be a no-op.
    """
    if followerId in self.followees:
      self.followees[followerId].discard(followeeId)


# Your Twitter object will be instantiated and called as such:
# obj = Twitter()
# obj.postTweet(userId,tweetId)
# param_2 = obj.getNewsFeed(userId)
# obj.follow(followerId,followeeId)
# obj.unfollow(followerId,followeeId)
