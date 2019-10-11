const { List } = require("../list/List.js");

// Stack first in last out.
class Stack {
  constructor() {
    this.list = new List();
  }

  // isEmpty returns whether stack is empty.
  // @return [boolean]
  isEmpty() {
    return this.list.Len() === 0;
  }

  // peek returns the stack top element.
  peek() {
    const e = this.list.Front();
    if (e === null) {
      return null;
    }
    return e.value;
  }

  // push pushes value to stack top;
  push(value) {
    this.list.PushFront(value);
  }

  // pop remove the top element from stack, and returns it.
  pop() {
    const e = this.list.Front();
    if (e === null) {
      return null;
    }
    this.list.Remove(e);
    return e.value;
  }

  toArray() {
    return this.list.toArray().map(elem => elem.value);
  }

  toString(callback) {
    return this.list.toString(callback);
  }
}

module.exports = Stack;
