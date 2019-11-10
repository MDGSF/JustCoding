/*
 * @lc app=leetcode.cn id=42 lang=javascript
 *
 * [42] 接雨水
 *
 * https://leetcode-cn.com/problems/trapping-rain-water/description/
 *
 * algorithms
 * Hard (46.55%)
 * Likes:    632
 * Dislikes: 0
 * Total Accepted:    33.2K
 * Total Submissions: 71K
 * Testcase Example:  '[0,1,0,2,1,0,1,3,2,1,2,1]'
 *
 * 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
 *
 *
 *
 * 上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 感谢
 * Marcos 贡献此图。
 *
 * 示例:
 *
 * 输入: [0,1,0,2,1,0,1,3,2,1,2,1]
 * 输出: 6
 *
 */

// @lc code=start
/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function(height) {
  /*
  直接按问题描述进行。对于数组中的每个元素，我们找出下雨后水能达到的最高位置，
  等于两边最大高度的较小值减去当前高度的值。
  */

  let result = 0;
  const n = height.length;
  for (let i = 1; i < n - 1; i += 1) {
    let maxLeft = 0;
    let maxRight = 0;
    for (let j = i; j >= 0; j -= 1) {
      maxLeft = Math.max(maxLeft, height[j]);
    }
    for (let j = i; j < n; j += 1) {
      maxRight = Math.max(maxRight, height[j]);
    }
    const curwater = Math.min(maxLeft, maxRight) - height[i];
    result += curwater;
  }
  return result;
};
// @lc code=end
