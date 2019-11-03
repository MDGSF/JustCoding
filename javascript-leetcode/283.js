/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var moveZeroes = function(nums) {
  let lastNonZeroIdx = 0;
  for (let i = 0; i < nums.length; i += 1) {
    if (nums[i] !== 0) {
      nums[lastNonZeroIdx] = nums[i];
      lastNonZeroIdx += 1;
    }
  }
  for (let i = lastNonZeroIdx; i < nums.length; i += 1) {
    nums[i] = 0;
  }
};
