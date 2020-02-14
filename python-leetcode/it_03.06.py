import collections
from typing import List


class AnimalShelf:

  def __init__(self):
    self.queue = []

  def enqueue(self, animal: List[int]) -> None:
    if animal[1] != 0 and animal[1] != 1:
      return
    self.queue.append(animal)

  def dequeueAny(self) -> List[int]:
    if len(self.queue) == 0:
      return [-1, -1]
    return self.queue.pop(0)

  def dequeueDog(self) -> List[int]:
    for i in range(len(self.queue)):
      animal = self.queue[i]
      if animal[1] == 1:
        del self.queue[i]
        return animal
    return [-1, -1]

  def dequeueCat(self) -> List[int]:
    for i in range(len(self.queue)):
      animal = self.queue[i]
      if animal[1] == 0:
        del self.queue[i]
        return animal
    return [-1, -1]


# Your AnimalShelf object will be instantiated and called as such:
# obj = AnimalShelf()
# obj.enqueue(animal)
# param_2 = obj.dequeueAny()
# param_3 = obj.dequeueDog()
# param_4 = obj.dequeueCat()


ops = ["AnimalShelf", "dequeueCat", "dequeueCat", "dequeueAny", "dequeueDog", "dequeueCat", "enqueue", "dequeueAny", "enqueue", "dequeueCat", "enqueue", "dequeueCat", "dequeueAny", "dequeueAny", "enqueue", "dequeueDog", "enqueue", "dequeueCat", "dequeueDog", "enqueue", "dequeueCat", "dequeueCat", "dequeueDog", "enqueue", "dequeueDog", "dequeueCat", "dequeueDog", "dequeueAny", "dequeueCat", "dequeueAny", "enqueue", "enqueue", "dequeueDog", "dequeueAny", "dequeueDog", "dequeueCat", "enqueue", "dequeueAny", "enqueue", "enqueue", "dequeueDog", "dequeueAny", "dequeueAny", "enqueue", "dequeueCat", "dequeueDog", "dequeueAny", "dequeueCat", "enqueue", "enqueue", "dequeueCat", "dequeueDog", "dequeueDog", "dequeueDog", "dequeueDog", "dequeueDog", "enqueue", "enqueue", "enqueue", "enqueue", "enqueue", "dequeueCat", "dequeueCat", "dequeueDog", "enqueue"]
opsArgs = [[], [], [], [], [], [], [[0, 1]], [], [[1, 0]], [], [[2, 1]], [], [], [], [[3, 1]], [], [[4, 0]], [], [], [[5, 0]], [], [], [], [[6, 0]], [], [], [], [], [], [], [[7, 1]], [[8, 1]], [], [], [], [], [[9, 1]], [], [[10, 1]], [[11, 1]], [], [], [], [[12, 0]], [], [], [], [], [[13, 0]], [[14, 0]], [], [], [], [], [], [], [[15, 1]], [[16, 1]], [[17, 0]], [[18, 1]], [[19, 1]], [], [], [], [[20, 1]]]

c = AnimalShelf()
count = 0
for i in range(len(ops)):
  op = ops[i]
  args = opsArgs[i]
  result = None
  count += 1
  # print(count, "start", op, args, result)

  if op == "AnimalShelf":
    continue
  elif op == "dequeueCat":
    result = c.dequeueCat()
  elif op == "dequeueDog":
    result = c.dequeueDog()
  elif op == "dequeueAny":
    result = c.dequeueAny()
  elif op == "enqueue":
    c.enqueue(args[0])

  print(count, "end", op, args, result)
