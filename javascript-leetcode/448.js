/**
 * @param {number[]} nums
 * @return {number[]}
 */
var findDisappearedNumbers = function(nums) {
  for (let i = 0; i < nums.length; i += 1) {
    let val = nums[i];
    let absVal = Math.abs(nums[i]);
    nums[absVal - 1] = -Math.abs(nums[absVal - 1]);
  }
  let result = [];
  for (let i = 0; i < nums.length; i += 1) {
    if (nums[i] > 0) {
      result.push(i + 1);
    }
  }
  return result;
};
