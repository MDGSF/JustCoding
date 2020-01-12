/*
 * @lc app=leetcode.cn id=152 lang=javascript
 *
 * [152] 乘积最大子序列
 *
 * https://leetcode-cn.com/problems/maximum-product-subarray/description/
 *
 * algorithms
 * Medium (32.25%)
 * Likes:    284
 * Dislikes: 0
 * Total Accepted:    22.9K
 * Total Submissions: 64.2K
 * Testcase Example:  '[2,3,-2,4]'
 *
 * 给定一个整数数组 nums ，找出一个序列中乘积最大的连续子序列（该序列至少包含一个数）。
 *
 * 示例 1:
 *
 * 输入: [2,3,-2,4]
 * 输出: 6
 * 解释: 子数组 [2,3] 有最大乘积 6。
 *
 *
 * 示例 2:
 *
 * 输入: [-2,0,-1]
 * 输出: 0
 * 解释: 结果不能为 2, 因为 [-2,-1] 不是子数组。
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var maxProduct = function(nums) {
  /*
  状态: dp[i][2]
  dp[i][0] 表示从 0->i (包括第i个节点)的正的最大值。
  dp[i][1] 表示从 0->i (包括第i个节点)的负的最大值，也就是最小值。

  dp[i][0] =
    if (nums[i] >= 0) dp[i-1][0]*nums[i]
    else dp[i-1][1]*nums[i]

  dp[i][1] =
    if (nums[i] >= 0) dp[i-1][1]*nums[i]
    else dp[i-1][0]*nums[1]

  结果: max(dp[i][0], dp[i-1][0], ..., dp[0][0])
  */

  if (nums.length === 0) {
    return 0;
  }
  let dp = new Array(2).fill(null).map(() => {
    return new Array(2).fill(null);
  });
  dp[0][0] = nums[0];
  dp[0][1] = nums[0];
  let result = dp[0][0];
  for (let i = 1; i < nums.length; i += 1) {
    let cur = i % 2;
    let pre = (i - 1) % 2;
    dp[cur][0] = Math.max(dp[pre][0]*nums[i], dp[pre][1]*nums[i], nums[i]);
    dp[cur][1] = Math.min(dp[pre][0]*nums[i], dp[pre][1]*nums[i], nums[i]);
    result = Math.max(result, dp[cur][0]);
  }
  return result;
};
// @lc code=end

