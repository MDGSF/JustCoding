const longestCommonSubstring = require("../longestCommonSubstring");

describe("longestCommonSubstring", () => {
  it("should find longest common substring between two strings", () => {
    expect(longestCommonSubstring("", "")).toBe("");
    expect(longestCommonSubstring("ABC", "")).toBe("");
    expect(longestCommonSubstring("", "ABC")).toBe("");
    expect(longestCommonSubstring("ABABC", "BABCA")).toBe("BABC");
  });
});
