const HeapSort = require("../HeapSort.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("HeapSort", () => {
  it("should sort array", () => {
    testSort(HeapSort);
  });
});
