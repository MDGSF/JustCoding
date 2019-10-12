const HashStrstr = require("../HashStrstr");

describe("HashStrstr", () => {
  it("should find word position in given text", () => {
    expect(HashStrstr("", "")).toBe(-1);
    expect(HashStrstr("a", "")).toBe(-1);
    expect(HashStrstr("a", "a")).toBe(0);
    expect(HashStrstr("abcbcglx", "abca")).toBe(-1);
    expect(HashStrstr("abcbcglx", "bcgl")).toBe(3);

    expect(HashStrstr("abcxabcdabxabcdabcdabcy", "abcdabcy")).toBe(15);
    expect(HashStrstr("abcxabcdabxabcdabcdabcy", "abcdabca")).toBe(-1);
    expect(HashStrstr("abcxabcdabxaabcdabcabcdabcdabcy", "abcdabca")).toBe(12);
    expect(HashStrstr("abcxabcdabxaabaabaaaabcdabcdabcy", "aabaabaaa")).toBe(
      11
    );
  });
});
