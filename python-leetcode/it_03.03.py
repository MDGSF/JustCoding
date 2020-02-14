class StackOfPlates:

  def __init__(self, cap: int):
    self.stack = []
    self.cap = cap

  def push(self, val: int) -> None:
    if len(self.stack) == 0 or len(self.stack[-1]) >= self.cap:
      self.stack.append([])
    last = self.stack[-1]
    last.append(val)

  def pop(self) -> int:
    while len(self.stack) > 0 and len(self.stack[-1]) == 0:
      self.stack.pop()
    if len(self.stack) == 0:
      return -1
    return self.stack[-1].pop()

  def popAt(self, index: int) -> int:
    if index < 0 or index >= len(self.stack):
      return -1
    if len(self.stack[index]) == 0:
      return -1
    return self.stack[index].pop()

# Your StackOfPlates object will be instantiated and called as such:
# obj = StackOfPlates(cap)
# obj.push(val)
# param_2 = obj.pop()
# param_3 = obj.popAt(index)
