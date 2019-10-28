const CountingSort = require("../CountingSort.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("CountingSort", () => {
  it("should sort array", () => {
    testSort(CountingSort);
  });
});
