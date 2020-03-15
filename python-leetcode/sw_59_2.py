class MaxQueue:

  def __init__(self):
    self.maxqueue = []
    self.queue = []

  def max_value(self) -> int:
    if len(self.maxqueue) == 0: return -1
    return self.maxqueue[0]

  def push_back(self, value: int) -> None:
    self.queue.append(value)
    for i in range(len(self.maxqueue) - 1, -1, -1):
      if self.maxqueue[i] < value:
        self.maxqueue.pop()
    self.maxqueue.append(value)

  def pop_front(self) -> int:
    if len(self.queue) == 0: return -1
    value = self.queue.pop(0)
    if value == self.maxqueue[0]:
      self.maxqueue.pop(0)
    return value


# Your MaxQueue object will be instantiated and called as such:
# obj = MaxQueue()
# param_1 = obj.max_value()
# obj.push_back(value)
# param_3 = obj.pop_front()

obj = MaxQueue()
obj.push_back(1)
obj.push_back(2)
val = obj.max_value()
print(val)
