/**
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
  let dp = new Array(nums.length).fill(0);
  dp[0] = nums[0];
  for (let i = 1; i < nums.length; i += 1) {
    dp[i] = Math.max(dp[i-1]+nums[i], nums[i]);
  }

  let maxsum = dp[0];
  for (let i = 1; i < dp.length; i += 1) {
    maxsum = Math.max(maxsum, dp[i]);
  }
  return maxsum;
};
