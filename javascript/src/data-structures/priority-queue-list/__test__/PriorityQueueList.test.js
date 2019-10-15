const PriorityQueueList = require("../PriorityQueueList");

describe("PriorityQueueList", () => {
  it("should create default priority queue", () => {
    const priorityQueue = new PriorityQueueList();
    expect(priorityQueue).toBeDefined();
  });

  it("should insert items to queue and respect priorities", () => {
    const priorityQueue = new PriorityQueueList();

    priorityQueue.push(10, 1);
    expect(priorityQueue.toString()).toBe("10:1");
    expect(priorityQueue.peek()).toBe(10);

    priorityQueue.push(5, 2);
    expect(priorityQueue.toString()).toBe("10:1,5:2");
    expect(priorityQueue.peek()).toBe(10);

    priorityQueue.push(100, 0);
    expect(priorityQueue.toString()).toBe("100:0,10:1,5:2");
    expect(priorityQueue.peek()).toBe(100);
  });

  it("should poll from queue with respect to priorities", () => {
    const priorityQueue = new PriorityQueueList();

    priorityQueue.push(10, 1);
    priorityQueue.push(5, 2);
    priorityQueue.push(100, 0);
    priorityQueue.push(200, 0);

    expect(priorityQueue.pop()).toBe(100);
    expect(priorityQueue.pop()).toBe(200);
    expect(priorityQueue.pop()).toBe(10);
    expect(priorityQueue.pop()).toBe(5);
  });
});
