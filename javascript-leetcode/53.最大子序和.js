/*
 * @lc app=leetcode.cn id=53 lang=javascript
 *
 * [53] 最大子序和
 *
 * https://leetcode-cn.com/problems/maximum-subarray/description/
 *
 * algorithms
 * Easy (47.66%)
 * Likes:    1300
 * Dislikes: 0
 * Total Accepted:    111.2K
 * Total Submissions: 232.5K
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
 *
 * 示例:
 *
 * 输入: [-2,1,-3,4,-1,2,1,-5,4],
 * 输出: 6
 * 解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。
 *
 *
 * 进阶:
 *
 * 如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的分治法求解。
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
  let maxSum = nums[0];
  let curSum = 0;
  for (let i = 0; i < nums.length; i += 1) {
    let num = nums[i];
    if (curSum > 0) {
      curSum += num;
    } else {
      curSum = num;
    }
    maxSum = Math.max(maxSum, curSum);
  }
  return maxSum;
};
// @lc code=end
