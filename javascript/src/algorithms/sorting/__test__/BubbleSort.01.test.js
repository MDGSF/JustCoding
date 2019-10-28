const BubbleSort = require("../BubbleSort.01.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("BubbleSort 01", () => {
  it("should sort array", () => {
    testSort(BubbleSort);
  });
});
