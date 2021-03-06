/*
 * @lc app=leetcode.cn id=62 lang=javascript
 *
 * [62] 不同路径
 *
 * https://leetcode-cn.com/problems/unique-paths/description/
 *
 * algorithms
 * Medium (56.74%)
 * Likes:    338
 * Dislikes: 0
 * Total Accepted:    46.6K
 * Total Submissions: 81.8K
 * Testcase Example:  '3\n2'
 *
 * 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
 *
 * 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
 *
 * 问总共有多少条不同的路径？
 *
 *
 *
 * 例如，上图是一个7 x 3 的网格。有多少可能的路径？
 *
 * 说明：m 和 n 的值均不超过 100。
 *
 * 示例 1:
 *
 * 输入: m = 3, n = 2
 * 输出: 3
 * 解释:
 * 从左上角开始，总共有 3 条路径可以到达右下角。
 * 1. 向右 -> 向右 -> 向下
 * 2. 向右 -> 向下 -> 向右
 * 3. 向下 -> 向右 -> 向右
 *
 *
 * 示例 2:
 *
 * 输入: m = 7, n = 3
 * 输出: 28
 *
 */

// @lc code=start
/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
var uniquePaths = function(m, n) {
  let rows = n;
  let cols = m;
  let dp = new Array(rows).fill(null).map(() => {
    return new Array(cols).fill(0);
  });
  dp[rows-1][cols-1] = 1;
  for (let row = rows - 2; row >= 0; row -= 1) {
    dp[row][cols-1] = 1;
  }
  for (let col = cols - 2; col >= 0; col -= 1) {
    dp[rows-1][col] = 1;
  }
  for (let row = rows - 2; row >= 0; row -= 1) {
    for (let col = cols - 2; col >= 0; col -= 1) {
      dp[row][col] = dp[row+1][col] + dp[row][col+1];
    }
  }
  return dp[0][0];
};
// @lc code=end

