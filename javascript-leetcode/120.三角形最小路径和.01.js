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
  const ctx = {};
  ctx.minSum = Number.MAX_VALUE;
  dfs(ctx, triangle, 0, 0, 0);
  return ctx.minSum;
};

// 这种 dfs 会超时
function dfs(ctx, triangle, row, col, sum) {
  // recursion terminator
  if (row >= triangle.length) {
    if (sum < ctx.minSum) {
      ctx.minSum = sum;
    }
    return;
  }

  // process logic in current level

  // drill down
  dfs(ctx, triangle, row + 1, col, sum + triangle[row][col]);
  dfs(ctx, triangle, row + 1, col + 1, sum + triangle[row][col]);

  // reverse the current level status if needed.
}
// @lc code=end

const triangle = [];
triangle.push([2]);
triangle.push([3, 4]);
triangle.push([6, 5, 7]);
triangle.push([4, 1, 8, 3]);

const result = minimumTotal(triangle);
console.log(result);
