const InsertionSort = require("../InsertionSort.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("InsertionSort", () => {
  it("should sort array", () => {
    testSort(InsertionSort);
  });
});
