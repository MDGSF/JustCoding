const assert = require("assert");
const { List, Element: ListElement } = require("../List.js");

function checkListPointers(l, es) {
  const { root } = l;

  expect(l.Len()).toBe(es.length);

  if (es.length === 0) {
    const err =
      (l.root.next !== null && l.root.next !== l.root) ||
      (l.root.prev !== null && l.root.prev !== l.root);
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

function checkList(l, es) {
  expect(l.Len()).toBe(es.length);

  let i = 0;
  for (let e = l.Front(); e !== null; e = e.Next()) {
    const le = e.value;
    assert.equal(le, es[i], `elt[${i}].value = ${le}, want = ${es[i]}`);
    i++;
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
      if (typeof e.value === "number") {
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

  it("extending", () => {
    const l1 = new List();
    const l2 = new List();

    l1.PushBack(1);
    l1.PushBack(2);
    l1.PushBack(3);

    l2.PushBack(4);
    l2.PushBack(5);

    let l3 = new List();
    l3.PushBackList(l1);
    checkList(l3, [1, 2, 3]);
    l3.PushBackList(l2);
    checkList(l3, [1, 2, 3, 4, 5]);

    l3 = new List();
    l3.PushFrontList(l2);
    checkList(l3, [4, 5]);
    l3.PushFrontList(l1);
    checkList(l3, [1, 2, 3, 4, 5]);

    checkList(l1, [1, 2, 3]);
    checkList(l2, [4, 5]);

    l3 = new List();
    l3.PushBackList(l1);
    checkList(l3, [1, 2, 3]);
    l3.PushBackList(l3);
    checkList(l3, [1, 2, 3, 1, 2, 3]);

    l3 = new List();
    l3.PushFrontList(l1);
    checkList(l3, [1, 2, 3]);
    l3.PushFrontList(l3);
    checkList(l3, [1, 2, 3, 1, 2, 3]);

    l3 = new List();
    l1.PushBackList(l3);
    checkList(l1, [1, 2, 3]);
    l1.PushFrontList(l3);
    checkList(l1, [1, 2, 3]);
  });

  it("test Remove", () => {
    const l = new List();
    const e1 = l.PushBack(1);
    const e2 = l.PushBack(2);
    checkListPointers(l, [e1, e2]);
    const e = l.Front();
    l.Remove(e);
    checkListPointers(l, [e2]);
    l.Remove(e);
    checkListPointers(l, [e2]);
  });

  it("test 4103", () => {
    const l1 = new List();
    l1.PushBack(1);
    l1.PushBack(2);

    const l2 = new List();
    l2.PushBack(3);
    l2.PushBack(4);

    const e = l1.Front();
    l2.Remove(e);
    expect(l2.Len()).toBe(2);

    l1.InsertBefore(8, e);
    expect(l1.Len()).toBe(3);
  });

  it("test 6349", () => {
    const l = new List();
    l.PushBack(1);
    l.PushBack(2);

    const e = l.Front();
    l.Remove(e);
    expect(e.value).toBe(1);
    expect(e.Next()).toBeNull();
    expect(e.Prev()).toBeNull();
  });

  it("should move success", () => {
    const l = new List();
    let e1 = l.PushBack(1);
    let e2 = l.PushBack(2);
    let e3 = l.PushBack(3);
    let e4 = l.PushBack(4);

    l.MoveAfter(e3, e3);
    checkListPointers(l, [e1, e2, e3, e4]);
    l.MoveBefore(e2, e2);
    checkListPointers(l, [e1, e2, e3, e4]);

    l.MoveAfter(e3, e2);
    checkListPointers(l, [e1, e2, e3, e4]);
    l.MoveBefore(e2, e3);
    checkListPointers(l, [e1, e2, e3, e4]);

    l.MoveBefore(e2, e4);
    checkListPointers(l, [e1, e3, e2, e4]);
    [e3] = [e2, (e2 = e3)];

    l.MoveBefore(e4, e1);
    checkListPointers(l, [e4, e1, e2, e3]);
    let e1bak = e1;
    let e2bak = e2;
    let e3bak = e3;
    let e4bak = e4;
    e1 = e4bak;
    e2 = e1bak;
    e3 = e2bak;
    e4 = e3bak;

    l.MoveAfter(e4, e1);
    checkListPointers(l, [e1, e4, e2, e3]);
    e1bak = e1;
    e2bak = e2;
    e3bak = e3;
    e4bak = e4;
    e1 = e1bak;
    e2 = e4bak;
    e3 = e2bak;
    e4 = e3bak;

    l.MoveAfter(e2, e3);
    checkListPointers(l, [e1, e3, e2, e4]);
  });

  it("test zero list", () => {
    const l1 = new List();
    l1.PushFront(1);
    checkList(l1, [1]);

    const l2 = new List();
    l2.PushFront(1);
    checkList(l2, [1]);

    const l3 = new List();
    l3.PushFrontList(l1);
    checkList(l3, [1]);

    const l4 = new List();
    l4.PushBackList(l2);
    checkList(l4, [1]);
  });

  it("test insert before unknown mark", () => {
    const l = new List();
    l.PushBack(1);
    l.PushBack(2);
    l.PushBack(3);
    const mark = new ListElement();
    l.InsertBefore(1, mark);
    checkList(l, [1, 2, 3]);
  });

  it("test insert after unknown mark", () => {
    const l = new List();
    l.PushBack(1);
    l.PushBack(2);
    l.PushBack(3);
    l.InsertAfter(1, new ListElement());
    checkList(l, [1, 2, 3]);
  });

  it("test move unknown mark", () => {
    const l1 = new List();
    const e1 = l1.PushBack(1);

    const l2 = new List();
    const e2 = l2.PushBack(2);

    l1.MoveAfter(e1, e2);
    checkList(l1, [1]);
    checkList(l2, [2]);

    l1.MoveBefore(e1, e2);
    checkList(l1, [1]);
    checkList(l2, [2]);
  });
});
