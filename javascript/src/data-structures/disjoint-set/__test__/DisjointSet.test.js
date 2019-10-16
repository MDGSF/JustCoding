const DisjointSet = require("../DisjointSet");

describe("DisjointSet", () => {
  it("should throw error when trying to union and check not existing sets", () => {
    function mergeNotExistingSets() {
      const disjointSet = new DisjointSet();

      disjointSet.union("A", "B");
    }

    function checkNotExistingSets() {
      const disjointSet = new DisjointSet();

      disjointSet.inSameSet("A", "B");
    }

    expect(mergeNotExistingSets).toThrow();
    expect(checkNotExistingSets).toThrow();
  });
});
