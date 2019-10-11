const LinkedList = require("../LinkedList");

describe("LinkedList", () => {
  it("should create empty linked list", () => {
    const linkedList = new LinkedList();
    expect(linkedList.toString()).toBe("");
    expect(linkedList.Len()).toBe(0);
  });

  it("should push front to linked list", () => {
    const linkedList = new LinkedList();

    linkedList.PushFront(1);
    expect(linkedList.Len()).toBe(1);
    expect(linkedList.Front()).toBe(1);
    expect(linkedList.Back()).toBe(1);

    linkedList.PushFront(2);
    linkedList.PushFront(3);
    expect(linkedList.Len()).toBe(3);
    expect(linkedList.Front()).toBe(3);
    expect(linkedList.Back()).toBe(1);
    expect(linkedList.toString()).toBe("3,2,1");
  });

  it("should push back to linked list", () => {
    const linkedList = new LinkedList();

    linkedList.PushBack(1);
    expect(linkedList.Len()).toBe(1);
    expect(linkedList.Front()).toBe(1);
    expect(linkedList.Back()).toBe(1);

    linkedList.PushBack(2);
    linkedList.PushBack(3);
    expect(linkedList.Len()).toBe(3);
    expect(linkedList.Front()).toBe(1);
    expect(linkedList.Back()).toBe(3);
    expect(linkedList.toString()).toBe("1,2,3");
  });

  it("should delete linked list tail", () => {
    const linkedList = new LinkedList();

    linkedList.PushBack(1);
    linkedList.PushBack(2);
    linkedList.PushBack(3);

    expect(linkedList.head.toString()).toBe("1");
    expect(linkedList.Tail().toString()).toBe("3");

    const deletedNode1 = linkedList.DeleteTail();

    expect(deletedNode1.value).toBe(3);
    expect(linkedList.toString()).toBe("1,2");
    expect(linkedList.head.toString()).toBe("1");
    expect(linkedList.Tail().toString()).toBe("2");

    const deletedNode2 = linkedList.DeleteTail();

    expect(deletedNode2.value).toBe(2);
    expect(linkedList.toString()).toBe("1");
    expect(linkedList.head.toString()).toBe("1");
    expect(linkedList.Tail().toString()).toBe("1");

    const deletedNode3 = linkedList.DeleteTail();

    expect(deletedNode3.value).toBe(1);
    expect(linkedList.toString()).toBe("");
    expect(linkedList.head).toBeNull();
    expect(linkedList.Tail()).toBeNull();
  });

  it("should delete linked list head", () => {
    const linkedList = new LinkedList();

    expect(linkedList.DeleteHead()).toBeNull();

    linkedList.PushBack(1);
    linkedList.PushBack(2);

    expect(linkedList.head.toString()).toBe("1");
    expect(linkedList.Tail().toString()).toBe("2");

    const deletedNode1 = linkedList.DeleteHead();

    expect(deletedNode1.value).toBe(1);
    expect(linkedList.toString()).toBe("2");
    expect(linkedList.head.toString()).toBe("2");
    expect(linkedList.Tail().toString()).toBe("2");

    const deletedNode2 = linkedList.DeleteHead();

    expect(deletedNode2.value).toBe(2);
    expect(linkedList.toString()).toBe("");
    expect(linkedList.head).toBeNull();
    expect(linkedList.Tail()).toBeNull();
  });

  it("should be possible to store objects in the list", () => {
    const linkedList = new LinkedList();

    const nodeValue1 = { value: 1, key: "key1" };
    const nodeValue2 = { value: 2, key: "key2" };

    linkedList.PushBack(nodeValue1);
    linkedList.PushFront(nodeValue2);

    const nodeStringifier = value => `${value.key}:${value.value}`;

    expect(linkedList.toString(nodeStringifier)).toBe("key2:2,key1:1");
  });
});
