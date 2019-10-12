const NMStrstr = require("../NMStrstr");

describe("NMStrstr", () => {
  it("should find word position in given text", () => {
    expect(NMStrstr("", "")).toBe(-1);
    expect(NMStrstr("a", "")).toBe(-1);
    expect(NMStrstr("a", "a")).toBe(0);
    expect(NMStrstr("abcbcglx", "abca")).toBe(-1);
    expect(NMStrstr("abcbcglx", "bcgl")).toBe(3);

    expect(NMStrstr("abcxabcdabxabcdabcdabcy", "abcdabcy")).toBe(15);
    expect(NMStrstr("abcxabcdabxabcdabcdabcy", "abcdabca")).toBe(-1);
    expect(NMStrstr("abcxabcdabxaabcdabcabcdabcdabcy", "abcdabca")).toBe(12);
    expect(NMStrstr("abcxabcdabxaabaabaaaabcdabcdabcy", "aabaabaaa")).toBe(11);
  });
});
