class CQueue:

  def __init__(self):
    self.stack1 = []
    self.stack2 = []

  def appendTail(self, value: int) -> None:
    self.stack1.append(value)

  def deleteHead(self) -> int:
    if len(self.stack1) == 0 and len(self.stack2) == 0:
      return -1
    if len(self.stack2) == 0:
      while len(self.stack1) > 0:
        self.stack2.append(self.stack1.pop())
    return self.stack2.pop()


# Your CQueue object will be instantiated and called as such:
# obj = CQueue()
# obj.appendTail(value)
# param_2 = obj.deleteHead()
