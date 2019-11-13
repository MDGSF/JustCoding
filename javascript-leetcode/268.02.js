/**
 * @param {number[]} nums
 * @return {number}
 */
var missingNumber = function(nums) {
  let expectSum = (nums.length * (nums.length + 1)) / 2;
  let actualSum = nums.reduce((acc, cur) => acc + cur);
  return expectSum - actualSum;
};
