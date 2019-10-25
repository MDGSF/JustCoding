const { List } = require("../list/List");
const hasOwnProperty = Object.prototype.hasOwnProperty;

class LRUCache {
  /*
  @param: {number} capacity of LRUCache
  */
  constructor(capacity) {
    this.capacity = capacity;
    this.m = {};
    this.list = new List();
  }

  put(key, value) {
    if (hasOwnProperty.call(this.m, key)) {
      let e = this.m[key];
      e.value.value = value;
      this.list.MoveToBack(e);
    } else {
      this.m[key] = this.list.PushBack({ key, value });
      if (this.list.Len() > this.capacity) {
        let front = this.list.Front();
        this.list.Remove(front);
        delete this.m[front.value.key];
      }
    }
  }

  get(key) {
    if (!hasOwnProperty.call(this.m, key)) {
      return -1;
    }
    let e = this.m[key];
    this.list.MoveToBack(e);
    return e.value.value;
  }
}

module.exports = LRUCache;
