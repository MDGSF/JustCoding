/**
 * @param {number[][]} nums
 * @param {number} r
 * @param {number} c
 * @return {number[][]}
 */
var matrixReshape = function(nums, r, c) {
  if (nums.length === 0 || nums[0].length === 0) {
    return nums;
  }
  let srcCount = nums.length * nums[0].length;
  let dstCount = r * c;
  if (srcCount !== dstCount) {
    return nums;
  }

  let newNums = Array(r)
    .fill(null)
    .map(() => {
      return Array(c).fill(null);
    });

  let srcRow = 0;
  let srcColumn = -1;
  for (let i = 0; i < r; i += 1) {
    for (let j = 0; j < c; j += 1) {
      if (srcColumn === nums[0].length - 1) {
        srcRow += 1;
        srcColumn = 0;
      } else {
        srcColumn += 1;
      }
      newNums[i][j] = nums[srcRow][srcColumn];
    }
  }
  return newNums;
};

const nums = [[1, 2], [3, 4]];
const result = matrixReshape(nums, 1, 4);
console.log(result);
