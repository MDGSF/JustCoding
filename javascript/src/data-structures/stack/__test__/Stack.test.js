const Stack = require("../Stack.js");

describe("Stack", () => {
  it("should be stack", () => {
    const s = new Stack();
    expect(s.isEmpty()).toBeTruthy();

    s.push(1);
    expect(s.isEmpty()).toBeFalsy();
    expect(s.peek()).toBe(1);

    s.push(2);
    expect(s.peek()).toBe(2);

    expect(s.pop()).toBe(2);
    expect(s.pop()).toBe(1);
    expect(s.pop()).toBeNull();
    expect(s.isEmpty()).toBeTruthy();
  });
});
