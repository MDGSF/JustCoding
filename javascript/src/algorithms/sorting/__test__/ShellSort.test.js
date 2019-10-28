const ShellSort = require("../ShellSort.js");
const {
  sortedArr,
  reverseArr,
  notSortedArr,
  equalArr,
  negativeArr,
  negativeArrSorted,
  testSort
} = require("../MySortTester.js");

describe("ShellSort", () => {
  it("should sort array", () => {
    testSort(ShellSort);
  });
});
