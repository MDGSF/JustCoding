/*
 * @lc app=leetcode.cn id=84 lang=javascript
 *
 * [84] 柱状图中最大的矩形
 *
 * https://leetcode-cn.com/problems/largest-rectangle-in-histogram/description/
 *
 * algorithms
 * Hard (37.24%)
 * Likes:    302
 * Dislikes: 0
 * Total Accepted:    16.6K
 * Total Submissions: 44.5K
 * Testcase Example:  '[2,1,5,6,2,3]'
 *
 * 给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
 *
 * 求在该柱状图中，能够勾勒出来的矩形的最大面积。
 *
 *
 *
 *
 *
 * 以上是柱状图的示例，其中每个柱子的宽度为 1，给定的高度为 [2,1,5,6,2,3]。
 *
 *
 *
 *
 *
 * 图中阴影部分为所能勾勒出的最大矩形面积，其面积为 10 个单位。
 *
 *
 *
 * 示例:
 *
 * 输入: [2,1,5,6,2,3]
 * 输出: 10
 *
 */

// @lc code=start
/**
 * @param {number[]} heights
 * @return {number}
 */
var largestRectangleArea = function(heights) {
  // 暴力解法，会超出时间限制。
  let maxarea = 0;
  const n = heights.length;
  for (let i = 0; i < n; i += 1) {
    for (let j = i; j < n; j += 1) {
      let minHeight = heights[i];
      for (let k = i; k <= j; k += 1) {
        if (heights[k] < minHeight) {
          minHeight = heights[k];
        }
      }
      const curarea = minHeight * (j - i + 1);
      if (curarea > maxarea) {
        maxarea = curarea;
      }
    }
  }
  return maxarea;
};
// @lc code=end
