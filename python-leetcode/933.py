class RecentCounter:

  def __init__(self):
    self.queue = []

  def ping(self, t: int) -> int:
    self.queue.append(t)
    while t - self.queue[0] > 3000:
      self.queue.pop(0)
    return len(self.queue)


# Your RecentCounter object will be instantiated and called as such:
# obj = RecentCounter()
# param_1 = obj.ping(t)
