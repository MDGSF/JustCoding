const isNumber = require("../isNumber");
describe("isNumber", () => {
  it("check valid number", () => {
    for (let i = 0; i < 10; i += 1) {
      const s = "" + i;
      expect(isNumber(s)).toBe(true);
    }
  });

  it("check invalid number", () => {
    expect(isNumber("a")).toBe(false);
    expect(isNumber("b")).toBe(false);
  });
});
