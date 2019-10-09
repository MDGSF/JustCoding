const assert = require("assert");
const { List, Element: ListElement } = require("../List.js");

function checkListPointers(l, es) {
  const { root } = l;

  expect(l.Len()).toBe(es.length);

  if (es.length === 0) {
    const err = (l.root.next !== null && l.root.next !== l.root)
      || (l.root.prev !== null && l.root.prev !== l.root);
    assert.equal(err, false, `${l.root}, ${l.root.prev}, ${l.root.next}`);
    return;
  }

  for (let i = 0; i < es.length; i++) {
    const e = es[i];
    let prev = root;
    let Prev = null;
    if (i > 0) {
      prev = es[i - 1];
      Prev = prev;
    }

    let p = e.prev;
    assert.equal(p, prev, `elt[${i}](${e}).prev = ${p}, want = ${prev}`);

    p = e.Prev();
    assert.equal(p, Prev, `elt[${i}](${e}).Prev() = ${p}, want = ${Prev}`);

    let next = root;
    let Next = null;
    if (i < es.length - 1) {
      next = es[i + 1];
      Next = next;
    }

    let n = e.next;
    assert.equal(n, next, `elt[${i}](${e}).next = ${n}, want = ${next}`);

    n = e.Next();
    assert.equal(n, Next, `elt[${i}](${e}).Next() = ${n}, want = ${Next}`);
  }
}

describe("List", () => {
  it("should create list element with value", () => {
    const elem = new ListElement(1);
    expect(elem.value).toBe(1);
    expect(elem.list).toBeNull();
    expect(elem.prev).toBeNull();
    expect(elem.next).toBeNull();
  });

  it("should link nodes together", () => {
    const node2 = new ListElement(2);
    const node1 = new ListElement(1, null, null, node2);

    expect(node1.next).toBeDefined();
    expect(node2.next).toBeNull();
    expect(node1.value).toBe(1);
    expect(node1.next.value).toBe(2);
  });

  it("should be List", () => {
    const l = new List();
    checkListPointers(l, []);

    const e = l.PushFront("a");
    checkListPointers(l, [e]);
    l.MoveToFront(e);
    checkListPointers(l, [e]);
    l.MoveToBack(e);
    checkListPointers(l, [e]);
    l.Remove(e);
    checkListPointers(l, []);

    let e2 = l.PushFront(2);
    const e1 = l.PushFront(1);
    const e3 = l.PushBack(3);
    const e4 = l.PushBack("banana");
    checkListPointers(l, [e1, e2, e3, e4]);

    l.Remove(e2);
    checkListPointers(l, [e1, e3, e4]);

    l.MoveToFront(e3);
    checkListPointers(l, [e3, e1, e4]);

    l.MoveToFront(e1);
    l.MoveToBack(e3);
    checkListPointers(l, [e1, e4, e3]);

    l.MoveToFront(e3);
    checkListPointers(l, [e3, e1, e4]);
    l.MoveToFront(e3);
    checkListPointers(l, [e3, e1, e4]);

    l.MoveToBack(e3);
    checkListPointers(l, [e1, e4, e3]);
    l.MoveToBack(e3);
    checkListPointers(l, [e1, e4, e3]);

    e2 = l.InsertBefore(2, e1);
    checkListPointers(l, [e2, e1, e4, e3]);
    l.Remove(e2);
    e2 = l.InsertBefore(2, e4);
    checkListPointers(l, [e1, e2, e4, e3]);
    l.Remove(e2);
    e2 = l.InsertBefore(2, e3);
    checkListPointers(l, [e1, e4, e2, e3]);
    l.Remove(e2);

    e2 = l.InsertAfter(2, e1);
    checkListPointers(l, [e1, e2, e4, e3]);
    l.Remove(e2);
    e2 = l.InsertAfter(2, e4);
    checkListPointers(l, [e1, e4, e2, e3]);
    l.Remove(e2);
    e2 = l.InsertAfter(2, e3);
    checkListPointers(l, [e1, e4, e3, e2]);
    l.Remove(e2);

    let sum = 0;
    for (let e = l.Front(); e !== null; e = e.Next()) {
      if (typeof e.value === 'number') {
        sum += e.value;
      }
    }
    expect(sum).toBe(4);

    let next = null;
    for (let e = l.Front(); e !== null; e = next) {
      next = e.Next();
      l.Remove(e);
    }
    checkListPointers(l, []);
  });
});
