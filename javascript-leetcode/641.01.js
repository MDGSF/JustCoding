class Element {
  constructor(value = null, list = null, prev = null, next = null) {
    this.value = value;
    this.list = list;
    this.prev = prev;
    this.next = next;
  }

  Next() {
    const p = this.next;
    if (this.list !== null && p !== this.list.root) {
      return p;
    }
    return null;
  }

  Prev() {
    const p = this.prev;
    if (this.list !== null && p !== this.list.root) {
      return p;
    }
    return null;
  }
}

/**
 * Initialize your data structure here. Set the size of the deque to be k.
 * @param {number} k
 */
var MyCircularDeque = function(k) {
  this.root = new Element();
  this.root.value = null;
  this.root.list = this;
  this.root.next = this.root;
  this.root.prev = this.root;
  this.len = 0;
  this.capacity = k;
};

/**
 * Adds an item at the front of Deque. Return true if the operation is successful.
 * @param {number} value
 * @return {boolean}
 */
MyCircularDeque.prototype.insertFront = function(value) {
  if (this.isFull()) {
    return false;
  }
  this.insertValue(value, this.root);
  return true;
};

/**
 * Adds an item at the rear of Deque. Return true if the operation is successful.
 * @param {number} value
 * @return {boolean}
 */
MyCircularDeque.prototype.insertLast = function(value) {
  if (this.isFull()) {
    return false;
  }
  this.insertValue(value, this.root.prev);
  return true;
};

MyCircularDeque.prototype.insertValue = function(v, at) {
  return this.insert(new Element(v), at);
};

MyCircularDeque.prototype.insert = function(e, at) {
  const n = at.next;
  at.next = e;
  e.prev = at;
  e.next = n;
  n.prev = e;
  e.list = this;
  this.len++;
  return e;
};

/**
 * Deletes an item from the front of Deque. Return true if the operation is successful.
 * @return {boolean}
 */
MyCircularDeque.prototype.deleteFront = function() {
  if (this.len === 0) {
    return false;
  }
  this.remove(this.root.next);
  return true;
};

/**
 * Deletes an item from the rear of Deque. Return true if the operation is successful.
 * @return {boolean}
 */
MyCircularDeque.prototype.deleteLast = function() {
  if (this.len === 0) {
    return false;
  }
  this.remove(this.root.prev);
  return true;
};

MyCircularDeque.prototype.remove = function(e) {
  e.prev.next = e.next;
  e.next.prev = e.prev;
  e.next = null;
  e.prev = null;
  e.list = null;
  this.len--;
  return e;
};

/**
 * Get the front item from the deque.
 * @return {number}
 */
MyCircularDeque.prototype.getFront = function() {
  if (this.len === 0) {
    return -1;
  }
  return this.root.next.value;
};

/**
 * Get the last item from the deque.
 * @return {number}
 */
MyCircularDeque.prototype.getRear = function() {
  if (this.len === 0) {
    return -1;
  }
  return this.root.prev.value;
};

/**
 * Checks whether the circular deque is empty or not.
 * @return {boolean}
 */
MyCircularDeque.prototype.isEmpty = function() {
  return this.len === 0;
};

/**
 * Checks whether the circular deque is full or not.
 * @return {boolean}
 */
MyCircularDeque.prototype.isFull = function() {
  return this.len === this.capacity;
};

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * var obj = new MyCircularDeque(k)
 * var param_1 = obj.insertFront(value)
 * var param_2 = obj.insertLast(value)
 * var param_3 = obj.deleteFront()
 * var param_4 = obj.deleteLast()
 * var param_5 = obj.getFront()
 * var param_6 = obj.getRear()
 * var param_7 = obj.isEmpty()
 * var param_8 = obj.isFull()
 */
