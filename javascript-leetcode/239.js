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
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
var maxSlidingWindow = function(nums, k) {
  let result = [];
  const list = new List();
  for (let i = 0; i < k; i += 1) {
    if (list.Len() === 0) {
      list.PushBack(nums[i]);
    } else {
      while (list.Len() > 0 && list.Back().value < nums[i]) {
        list.Remove(list.Back());
      }
      list.PushBack(nums[i]);
    }
  }
  result.push(list.Front().value);

  for (let i = k; i < nums.length; i += 1) {
    while (list.Len() > 0 && list.Back().value < nums[i]) {
      list.Remove(list.Back());
    }
    list.PushBack(nums[i]);
    if (list.Len() > k) {
      list.Remove(list.Front());
    }
    result.push(list.Front().value);
  }

  return result;
};

/*
输入: nums = [1,3,-1,-3,5,3,6,7], 和 k = 3
输出: [3,3,5,5,6,7]
const result = maxSlidingWindow([1, 3, -1, -3, 5, 3, 6, 7], 3);
console.log(result);
*/
