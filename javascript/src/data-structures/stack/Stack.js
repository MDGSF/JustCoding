const { List, Element: ListElement } = require("../list/List.js");

class Stack {
  constructor() {
    this.list = new List();
  }

  isEmpty() {
    return this.list.Len() === 0;
  }

  peek() {
    const e = this.list.Front();
    if (e === null) {
      return null;
    }
    return e.value;
  }

  push(value) {
    this.list.PushFront(value);
  }

  pop() {
    const e = this.list.Front();
    if (e === null) {
      return null;
    }
    this.list.Remove(e);
    return e.value;
  }
}

module.exports = Stack;
