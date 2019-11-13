/**
 * @param {number[]} nums
 * @return {number}
 */
var missingNumber = function(nums) {
  let sum = nums.length;

  for (let i = 0; i < nums.length; i += 1) {
    sum += i;
    sum -= nums[i];
  }

  return sum;
};
