const LRUCache = require("../LRUCache");

describe("LRUCache", () => {
  it("should create LRUCache", () => {
    let cache = new LRUCache(2);

    cache.put(1, 1);
    cache.put(2, 2);
    expect(cache.get(1)).toBe(1);
    cache.put(3, 3); // 该操作会使得密钥 2 作废
    expect(cache.get(2)).toBe(-1);
    cache.put(4, 4); // 该操作会使得密钥 1 作废
    expect(cache.get(1)).toBe(-1);
    expect(cache.get(3)).toBe(3);
    expect(cache.get(4)).toBe(4);
  });

  it("should expire", () => {
    let cache = new LRUCache(2);
    for (let i = 0; i < 10; i += 1) {
      cache.put(i, i);
    }
    expect(cache.get(9)).toBe(9);
    expect(cache.get(8)).toBe(8);
  });
});
