const QuickSort = require("../QuickSort.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("QuickSort", () => {
  it("should sort array", () => {
    testSort(QuickSort);
  });
});
