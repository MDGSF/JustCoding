/*
 * @lc app=leetcode.cn id=120 lang=javascript
 *
 * [120] 三角形最小路径和
 *
 * https://leetcode-cn.com/problems/triangle/description/
 *
 * algorithms
 * Medium (56.82%)
 * Likes:    237
 * Dislikes: 0
 * Total Accepted:    26.3K
 * Total Submissions: 42.2K
 * Testcase Example:  '[[2],[3,4],[6,5,7],[4,1,8,3]]'
 *
 * 给定一个三角形，找出自顶向下的最小路径和。每一步只能移动到下一行中相邻的结点上。
 *
 * 例如，给定三角形：
 *
 * [
 * ⁠    [2],
 * ⁠   [3,4],
 * ⁠  [6,5,7],
 * ⁠ [4,1,8,3]
 * ]
 *
 *
 * 自顶向下的最小路径和为 11（即，2 + 3 + 5 + 1 = 11）。
 *
 * 说明：
 *
 * 如果你可以只使用 O(n) 的额外空间（n 为三角形的总行数）来解决这个问题，那么你的算法会很加分。
 *
 */

// @lc code=start
/**
 * @param {number[][]} triangle
 * @return {number}
 */
var minimumTotal = function(triangle) {
  // dp 状态转移方程
  // dp[row][col] = triangle[row][col] + min(dp[row+1][col] + dp[row+1][col+1])
  let row = triangle.length - 2;
  while (row >= 0) {
    for (let col = 0; col <= row; col += 1) {
      triangle[row][col] =
        triangle[row][col] +
        Math.min(triangle[row + 1][col], triangle[row + 1][col + 1]);
    }
    row -= 1;
  }
  return triangle[0][0];
};

// @lc code=end

const triangle = [];
triangle.push([2]);
triangle.push([3, 4]);
triangle.push([6, 5, 7]);
triangle.push([4, 1, 8, 3]);

const result = minimumTotal(triangle);
console.log(result);
