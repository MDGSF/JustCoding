/*
 * @lc app=leetcode.cn id=52 lang=javascript
 *
 * [52] N皇后 II
 *
 * https://leetcode-cn.com/problems/n-queens-ii/description/
 *
 * algorithms
 * Hard (75.39%)
 * Likes:    79
 * Dislikes: 0
 * Total Accepted:    10.6K
 * Total Submissions: 14K
 * Testcase Example:  '4'
 *
 * n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
 *
 *
 *
 * 上图为 8 皇后问题的一种解法。
 *
 * 给定一个整数 n，返回 n 皇后不同的解决方案的数量。
 *
 * 示例:
 *
 * 输入: 4
 * 输出: 2
 * 解释: 4 皇后问题存在如下两个不同的解法。
 * [
 * [".Q..",  // 解法 1
 * "...Q",
 * "Q...",
 * "..Q."],
 *
 * ["..Q.",  // 解法 2
 * "Q...",
 * "...Q",
 * ".Q.."]
 * ]
 *
 *
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var totalNQueens = function(n) {
  const ctx = {};
  ctx.n = n;
  ctx.cols = new Set();
  ctx.pie = new Set();
  ctx.na = new Set();
  ctx.row = 0;
  ctx.curResult = [];
  ctx.result = [];
  dfs(ctx);

  for (let i = 0; i < ctx.result.length; i += 1) {
    for (let j = 0; j < ctx.result[i].length; j += 1) {
      const x = ctx.result[i][j];
      ctx.result[i][j] = '.'.repeat(x) + 'Q' + '.'.repeat(n - x - 1);
    }
  }
  return ctx.result.length;
};

function dfs(ctx) {
  if (ctx.row >= ctx.n) {
    ctx.result.push(ctx.curResult.slice(0));
    return;
  }

  for (let col = 0; col < ctx.n; col += 1) {
    if (
      ctx.cols.has(col) ||
      ctx.pie.has(ctx.row + col) ||
      ctx.na.has(ctx.row - col)
    ) {
      continue;
    }

    ctx.cols.add(col);
    ctx.pie.add(ctx.row + col);
    ctx.na.add(ctx.row - col);
    ctx.row += 1;
    ctx.curResult.push(col);

    dfs(ctx);

    ctx.curResult.pop();
    ctx.row -= 1;
    ctx.cols.delete(col);
    ctx.pie.delete(ctx.row + col);
    ctx.na.delete(ctx.row - col);
  }
}

const result = totalNQueens(4);
console.log(result);
// @lc code=end
