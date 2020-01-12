/*
 * @lc app=leetcode.cn id=63 lang=javascript
 *
 * [63] 不同路径 II
 *
 * https://leetcode-cn.com/problems/unique-paths-ii/description/
 *
 * algorithms
 * Medium (31.78%)
 * Likes:    172
 * Dislikes: 0
 * Total Accepted:    26.2K
 * Total Submissions: 82.2K
 * Testcase Example:  '[[0,0,0],[0,1,0],[0,0,0]]'
 *
 * 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
 *
 * 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
 *
 * 现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？
 *
 *
 *
 * 网格中的障碍物和空位置分别用 1 和 0 来表示。
 *
 * 说明：m 和 n 的值均不超过 100。
 *
 * 示例 1:
 *
 * 输入:
 * [
 * [0,0,0],
 * [0,1,0],
 * [0,0,0]
 * ]
 * 输出: 2
 * 解释:
 * 3x3 网格的正中间有一个障碍物。
 * 从左上角到右下角一共有 2 条不同的路径：
 * 1. 向右 -> 向右 -> 向下 -> 向下
 * 2. 向下 -> 向下 -> 向右 -> 向右
 *
 *
 */

// @lc code=start
/**
 * @param {number[][]} obstacleGrid
 * @return {number}
 */
var uniquePathsWithObstacles = function(obstacleGrid) {
  let rows = obstacleGrid.length;
  let cols = obstacleGrid[0].length;
  let dp = new Array(rows).fill(null).map(() => {
    return new Array(cols).fill(0);
  });

  dp[rows-1][cols-1] = obstacleGrid[rows-1][cols-1] === 0 ? 1 : 0;

  for (let row = rows - 2; row >= 0; row -= 1) {
    dp[row][cols-1] = (obstacleGrid[row][cols-1] === 0
      && dp[row+1][cols-1] === 1) ? 1 : 0;
  }

  for (let col = cols - 2; col >= 0; col -= 1) {
    dp[rows-1][col] = (obstacleGrid[rows-1][col] === 0
      && dp[rows-1][col+1] === 1) ? 1 : 0;
  }

  for (let row = rows - 2; row >= 0; row -= 1) {
    for (let col = cols - 2; col >= 0; col -= 1) {
      if (obstacleGrid[row][col] === 1) {
        dp[row][col] = 0;
      } else {
        dp[row][col] = dp[row+1][col] + dp[row][col+1];
      }
    }
  }

  return dp[0][0];
};
// @lc code=end


const obstacleGrid = [
  [0,0,0],
  [0,1,0],
  [0,0,0]
];
const result = uniquePathsWithObstacles(obstacleGrid);
console.log(result);

