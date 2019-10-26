/*
 * @lc app=leetcode.cn id=146 lang=javascript
 *
 * [146] LRU缓存机制
 *
 * https://leetcode-cn.com/problems/lru-cache/description/
 *
 * algorithms
 * Hard (38.90%)
 * Likes:    273
 * Dislikes: 0
 * Total Accepted:    21K
 * Total Submissions: 47.7K
 * Testcase Example:  '["LRUCache","put","put","get","put","get","put","get","get","get"]\n[[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]'
 *
 * 运用你所掌握的数据结构，设计和实现一个  LRU (最近最少使用) 缓存机制。它应该支持以下操作： 获取数据 get 和 写入数据 put 。
 *
 * 获取数据 get(key) - 如果密钥 (key) 存在于缓存中，则获取密钥的值（总是正数），否则返回 -1。
 * 写入数据 put(key, value) -
 * 如果密钥不存在，则写入其数据值。当缓存容量达到上限时，它应该在写入新数据之前删除最近最少使用的数据值，从而为新的数据值留出空间。
 *
 * 进阶:
 *
 * 你是否可以在 O(1) 时间复杂度内完成这两种操作？
 *
 * 示例:
 *
 * LRUCache cache = new LRUCache( 2 ); //缓存容量
 *
 * cache.put(1, 1);
 * cache.put(2, 2);
 * cache.get(1);       // 返回  1
 * cache.put(3, 3);    // 该操作会使得密钥 2 作废
 * cache.get(2);       // 返回 -1 (未找到)
 * cache.put(4, 4);    // 该操作会使得密钥 1 作废
 * cache.get(1);       // 返回 -1 (未找到)
 * cache.get(3);       // 返回  3
 * cache.get(4);       // 返回  4
 *
 *
 */

// @lc code=start
/**
 * @param {number} capacity
 */
var LRUCache = function(capacity) {
  this.capacity = capacity;
  this.m = {};
  this.list = new List();
};

/**
 * @param {number} key
 * @return {number}
 */
LRUCache.prototype.get = function(key) {
  if (!hasOwnProperty.call(this.m, key)) {
    return -1;
  }
  let e = this.m[key];
  this.list.MoveToBack(e);
  return e.value.value;
};

/**
 * @param {number} key
 * @param {number} value
 * @return {void}
 */
LRUCache.prototype.put = function(key, value) {
  if (hasOwnProperty.call(this.m, key)) {
    let e = this.m[key];
    e.value.value = value;
    this.list.MoveToBack(e);
  } else {
    this.m[key] = this.list.PushBack({key, value});
    if (this.list.Len() > this.capacity) {
      let front = this.list.Front();
      this.list.Remove(front);
      delete this.m[front.value.key];
    }
  }
};

// ---------------------------------------------------
// list implement

// Element is an element of a linked list.
class Element {
  constructor(value = null, list = null, prev = null, next = null) {
    this.value = value;
    this.list = list;
    this.prev = prev;
    this.next = next;
  }

  // Next returns the next list element or null.
  Next() {
    const p = this.next;
    if (this.list !== null && p !== this.list.root) {
      return p;
    }
    return null;
  }

  // Prev returns the previous list element or null.
  Prev() {
    const p = this.prev;
    if (this.list !== null && p !== this.list.root) {
      return p;
    }
    return null;
  }

  toString(callback) {
    return callback ? callback(this.value) : `${this.value}`;
  }
}

// List represents a doubly linked list.
class List {
  // this.root is a dumb node.
  constructor() {
    this.root = new Element();
    this.root.value = null;
    this.root.list = this;
    this.root.next = this.root;
    this.root.prev = this.root;
    this.len = 0;
  }

  // Len returns the number of elements of list.
  Len() {
    return this.len;
  }

  // Front returns the first element of list or nil if list is empty.
  // @return e [Element]
  Front() {
    if (this.len === 0) {
      return null;
    }
    return this.root.next;
  }

  // Back returns the last element of list or nil if list is empty.
  // @return e [Element]
  Back() {
    if (this.len === 0) {
      return null;
    }
    return this.root.prev;
  }

  // insert inserts e after at, increments this.len, and return e.
  // @param e [Element]
  // @param at [Element]
  // @return e [Element]
  insert(e, at) {
    const n = at.next;
    at.next = e;
    e.prev = at;
    e.next = n;
    n.prev = e;
    e.list = this;
    this.len++;
    return e;
  }

  // insertValue is a convenience wapper for insert(Element{value: v}, at).
  // @param v [Element.value]
  // @param at [Element]
  // @return e [Element]
  insertValue(v, at) {
    return this.insert(new Element(v), at);
  }

  // remove removes e from its list, decrements this.len, and return e.
  // @param e [Element]
  // @return e [Element]
  remove(e) {
    e.prev.next = e.next;
    e.next.prev = e.prev;
    e.next = null;
    e.prev = null;
    e.list = null;
    this.len--;
    return e;
  }

  // move moves e to next to at and returns e.
  // @param e [Element]
  // @param at [Element]
  // @return e [Element]
  move(e, at) {
    if (e === at) {
      return e;
    }
    e.prev.next = e.next;
    e.next.prev = e.prev;

    const n = at.next;
    at.next = e;
    e.prev = at;
    e.next = n;
    n.prev = e;

    return e;
  }

  // Remove removes e from list if e is an element of list.
  // @param e [Element]
  // @return v [Element.value]
  Remove(e) {
    if (e.list === this) {
      this.remove(e);
    }
    return e.value;
  }

  // PushFront inserts a new element e with value v at the front of list and
  // returns e.
  // @param v [Element.value]
  // @return e [Element]
  PushFront(v) {
    return this.insertValue(v, this.root);
  }

  // PushBack inserts a new element e with value v at the back of list and
  // returns e.
  // @param v [Element.value]
  // @return e [Element]
  PushBack(v) {
    return this.insertValue(v, this.root.prev);
  }

  // InsertBefore inserts a new element e with value v immediately before mark
  // and returns e.
  // @param v [Element.value]
  // @param mark [Element]
  // @return e [Element]
  InsertBefore(v, mark) {
    if (mark.list !== this) {
      return null;
    }
    return this.insertValue(v, mark.prev);
  }

  // InsertAfter inserts a new element e with value v immediately after mark
  // and returns e.
  // @param v [Element.value]
  // @param mark [Element]
  // @return e [Element]
  InsertAfter(v, mark) {
    if (mark.list !== this) {
      return null;
    }
    return this.insertValue(v, mark);
  }

  // MoveToFront moves element e to the front of list.
  // @param e [Element]
  MoveToFront(e) {
    if (e.list !== this || this.root.next === e) {
      return;
    }
    this.move(e, this.root);
  }

  // MoveToBack moves element e to the back of list.
  // @param e [Element]
  MoveToBack(e) {
    if (e.list !== this || this.root.prev === e) {
      return;
    }
    this.move(e, this.root.prev);
  }

  // MoveBefore moves element e to its new position before mark.
  // @param e [Element]
  // @param mark [Element]
  MoveBefore(e, mark) {
    if (e.list !== this || e === mark || mark.list !== this) {
      return;
    }
    this.move(e, mark.prev);
  }

  // MoveAfter moves element e to its new position after mark.
  // @param e [Element]
  // @param mark [Element]
  MoveAfter(e, mark) {
    if (e.list !== this || e === mark || mark.list !== this) {
      return;
    }
    this.move(e, mark);
  }

  // PushBackList inserts a copy of an other list at the back of list this.
  // @param other [List]
  PushBackList(other) {
    let i = other.Len();
    let e = other.Front();
    while (i > 0) {
      this.insertValue(e.value, this.root.prev);
      i--;
      e = e.Next();
    }
  }

  // PushFrontList inserts a copy of an other list at the front of list this.
  // @param other [List]
  PushFrontList(other) {
    let i = other.Len();
    let e = other.Back();
    while (i > 0) {
      this.insertValue(e.value, this.root);
      i--;
      e = e.Prev();
    }
  }

  // toArray push all elements into an array and returns.
  toArray() {
    const nodes = [];
    for (let e = this.Front(); e != null; e = e.Next()) {
      nodes.push(e);
    }
    return nodes;
  }

  toString(callback) {
    return this.toArray()
      .map(element => element.toString(callback))
      .toString();
  }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */
// @lc code=end
