/*
 * @lc app=leetcode.cn id=64 lang=javascript
 *
 * [64] 最小路径和
 *
 * https://leetcode-cn.com/problems/minimum-path-sum/description/
 *
 * algorithms
 * Medium (62.52%)
 * Likes:    305
 * Dislikes: 0
 * Total Accepted:    37.5K
 * Total Submissions: 59.8K
 * Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
 *
 * 给定一个包含非负整数的 m x n 网格，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
 *
 * 说明：每次只能向下或者向右移动一步。
 *
 * 示例:
 *
 * 输入:
 * [
 * [1,3,1],
 * ⁠ [1,5,1],
 * ⁠ [4,2,1]
 * ]
 * 输出: 7
 * 解释: 因为路径 1→3→1→1→1 的总和最小。
 *
 *
 */

// @lc code=start
/**
 * @param {number[][]} grid
 * @return {number}
 */
var minPathSum = function(grid) {
  // dp[i][j] = grid[i][j] + min(dp[i+1][j]+dp[i][j+1])
  let rows = grid.length;
  let cols = grid[0].length;
  for (let row = rows - 2; row >= 0; row -= 1) {
    grid[row][cols-1] += grid[row+1][cols-1];
  }
  for (let col = cols - 2; col >= 0; col -= 1) {
    grid[rows-1][col] += grid[rows-1][col+1];
  }
  for (let row = rows - 2; row >= 0; row -= 1) {
    for (let col = cols - 2; col >= 0; col -= 1) {
      grid[row][col] = grid[row][col] +
        Math.min(grid[row+1][col], grid[row][col+1]);
    }
  }
  return grid[0][0];
};
// @lc code=end

