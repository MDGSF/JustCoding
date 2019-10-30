const topk = require("../topk_partition");

describe("topk partition", () => {
  it("should get top k", () => {
    const a = [1, 2, 3, 4];
    expect(topk(a, 0)).toBe(null);
    expect(topk(a, 1)).toBe(4);
    expect(topk(a, 2)).toBe(3);
    expect(topk(a, 3)).toBe(2);
    expect(topk(a, 4)).toBe(1);
    expect(topk(a, 5)).toBe(null);

    expect(topk([1], 1)).toBe(1);

    expect(topk([1024, 512, 1, 10, 20, 100], 1)).toBe(1024);
    expect(topk([1024, 512, 1, 10, 20, 100], 2)).toBe(512);
    expect(topk([1024, 512, 1, 10, 20, 100], 3)).toBe(100);
  });
});
