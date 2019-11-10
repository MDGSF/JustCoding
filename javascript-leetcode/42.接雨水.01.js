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
  let i = 0;
  let j = height.length - 1;
  let result = 0;
  let bucket = 0;
  while (i < j) {
    let block = Math.min(height[i], height[j]);
    if (block > bucket) {
      bucket = block;
    }
    if (height[i] < height[j]) {
      result += bucket - height[i];
      i += 1;
    } else {
      result += bucket - height[j];
      j -= 1;
    }
  }
  return result;
};
// @lc code=end
