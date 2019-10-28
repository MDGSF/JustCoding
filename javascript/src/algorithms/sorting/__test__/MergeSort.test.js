const MergeSort = require("../MergeSort.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("MergeSort", () => {
  it("should sort array", () => {
    testSort(MergeSort);
  });
});
