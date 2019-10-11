const LinkedListNode = require("./LinkedListNode");

class LinkedList {
  constructor() {
    this.head = null;
  }

  // Len returns the number of elements of list.
  Len() {
    let len = 0;
    for (let p = this.head; p !== null; p = p.next) {
      len++;
    }
    return len;
  }

  // Front returns the first node value of list or nil if list is empty.
  Front() {
    if (this.head === null) {
      return null;
    }
    return this.head.value;
  }

  // Back returns the last node value of list or nil if list is empty.
  Back() {
    const tail = this.Tail();
    if (tail === null) {
      return null;
    }
    return tail.value;
  }

  // Tail get a pointer to tail node.
  Tail() {
    if (this.head === null) {
      return null;
    }
    let p = this.head;
    while (p.next !== null) {
      p = p.next;
    }
    return p;
  }

  PushFront(value) {
    const newNode = new LinkedListNode(value, this.head);
    this.head = newNode;
    return this;
  }

  PushBack(value) {
    const newNode = new LinkedListNode(value);
    if (this.head === null) {
      this.head = newNode;
    } else {
      const tail = this.Tail();
      tail.next = newNode;
    }
    return this;
  }

  DeleteTail() {
    if (this.head === null) {
      return null;
    }

    if (this.head.next === null) {
      // There is only one node in linked list.
      const deletedTail = this.head;
      this.head = null;
      return deletedTail;
    }

    // If there are many nodes in linked list.

    const deletedTail = this.Tail();

    let p = this.head;
    while (p.next !== null) {
      if (p.next.next === null) {
        p.next = null;
      } else {
        p = p.next;
      }
    }

    return deletedTail;
  }

  DeleteHead() {
    if (this.head === null) {
      return null;
    }

    const deletedHead = this.head;

    this.head = this.head.next;

    return deletedHead;
  }

  fromArray(values) {
    for (let i = values.length - 1; i >= 0; i--) {
      this.PushFront(values[i]);
    }
  }

  toArray() {
    const nodes = [];

    let p = this.head;
    while (p !== null) {
      nodes.push(p.value);
      p = p.next;
    }

    return nodes;
  }

  toString(callback) {
    return this.toArray()
      .map(node => node.toString(callback))
      .toString();
  }

  reverse() {
    let currNode = this.head;
    let prevNode = null;
    let nextNode = null;

    while (currNode) {
      nextNode = currNode.next;

      currNode.next = prevNode;

      prevNode = currNode;
      currNode = nextNode;
    }

    this.head = prevNode;

    return this;
  }
}

module.exports = LinkedList;
