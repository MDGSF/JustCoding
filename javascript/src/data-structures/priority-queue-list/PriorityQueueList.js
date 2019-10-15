const { List } = require("../list/List.js");

class PriorityQueueList {
  constructor() {
    this.list = new List();
  }

  push(item, priority = 0) {
    const newValue = { item, priority };
    if (this.list.Len() === 0) {
      this.list.PushBack(newValue);
      return;
    }

    let mark = null;
    for (let e = this.list.Front(); e != null; e = e.Next()) {
      if (e.value.priority > priority) {
        mark = e;
        break;
      }
    }

    if (mark === null) {
      this.list.PushBack(newValue);
    } else {
      this.list.InsertBefore(newValue, mark);
    }
  }

  peek() {
    if (this.list.Len() === 0) {
      return null;
    }
    return this.list.Front().value.item;
  }

  pop() {
    const e = this.list.Front();
    if (e === null) {
      return null;
    }
    return this.list.Remove(e).item;
  }

  toString() {
    const nodeStringifier = value => `${value.item}:${value.priority}`;
    return this.list.toString(nodeStringifier);
  }
}

module.exports = PriorityQueueList;
