/*
 * @lc app=leetcode.cn id=200 lang=javascript
 *
 * [200] 岛屿的个数
 *
 * https://leetcode-cn.com/problems/number-of-islands/description/
 *
 * algorithms
 * Medium (41.18%)
 * Likes:    258
 * Dislikes: 0
 * Total Accepted:    33.7K
 * Total Submissions: 73.7K
 * Testcase Example:  '[["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]'
 *
 * 给定一个由 '1'（陆地）和
 * '0'（水）组成的的二维网格，计算岛屿的数量。一个岛被水包围，并且它是通过水平方向或垂直方向上相邻的陆地连接而成的。你可以假设网格的四个边均被水包围。
 *
 * 示例 1:
 *
 * 输入:
 * 11110
 * 11010
 * 11000
 * 00000
 *
 * 输出: 1
 *
 *
 * 示例 2:
 *
 * 输入:
 * 11000
 * 11000
 * 00100
 * 00011
 *
 * 输出: 3
 *
 *
 */

// @lc code=start
/**
 * @param {character[][]} grid
 * @return {number}
 */
var numIslands = function(grid) {
  if (grid.length === 0) { return 0; }
  let rows = grid.length;
  let cols = grid[0].length;
  let count = 0;
  for (let row = 0; row < rows; row += 1) {
    for (let col = 0; col < cols; col += 1) {
      if (grid[row][col] === "1") {
        count += 1;
        destroyIslandDFS(grid, row, col);
      }
    }
  }
  return count;
};

function destroyIslandDFS(grid, row, col) {
  grid[row][col] = "0";
  const dirs = [ [-1, 0], [1, 0], [0, -1], [0, 1] ];
  for (let i = 0; i < dirs.length; i += 1) {
    let newRow = row + dirs[i][0];
    let newCol = col + dirs[i][1];
    if (isIndexValid(grid, newRow, newCol) &&
      grid[newRow][newCol] === "1") {
      destroyIslandDFS(grid, newRow, newCol);
    }
  }
}

function isIndexValid(grid, row, col) {
  if (row < 0 ||
      col < 0 ||
      row >= grid.length ||
      col >= grid[0].length) {
    return false;
  }
  return true;
}
// @lc code=end

