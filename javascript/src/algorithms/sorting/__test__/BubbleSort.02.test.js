const BubbleSort = require("../BubbleSort.02.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("BubbleSort 02", () => {
  it("should sort array", () => {
    testSort(BubbleSort);
  });
});
