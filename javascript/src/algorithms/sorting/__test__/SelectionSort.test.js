const SelectionSort = require("../SelectionSort.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("SelectionSort", () => {
  it("should sort array", () => {
    testSort(SelectionSort);
  });
});
