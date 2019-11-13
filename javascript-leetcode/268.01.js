/**
 * @param {number[]} nums
 * @return {number}
 */
var missingNumber = function(nums) {
  let missing = nums.length;
  for (let i = 0; i < nums.length; i += 1) {
    missing = missing ^ i ^ nums[i];
  }
  return missing;
};
