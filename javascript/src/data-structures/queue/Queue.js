const { List } = require("../list/List.js");

// Queue first in first out.
class Queue {
  constructor() {
    this.list = new List();
  }

  // isEmpty returns whether queue is empty.
  // @return [boolean]
  isEmpty() {
    return this.list.Len() === 0;
  }

  // peek returns the queue head element.
  peek() {
    const e = this.list.Front();
    if (e === null) {
      return null;
    }
    return e.value;
  }

  // enqueue add value to the end of the queue.
  enqueue(value) {
    this.list.PushBack(value);
  }

  // dequeue remove the element at the front of the queue, and returns it.
  dequeue() {
    const e = this.list.Front();
    if (e === null) {
      return null;
    }
    this.list.Remove(e);
    return e.value;
  }

  toString(callback) {
    return this.list.toString(callback);
  }
}

module.exports = Queue;
