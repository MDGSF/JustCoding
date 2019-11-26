/*
 * @lc app=leetcode.cn id=22 lang=javascript
 *
 * [22] 括号生成
 *
 * https://leetcode-cn.com/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (72.12%)
 * Likes:    579
 * Dislikes: 0
 * Total Accepted:    48.4K
 * Total Submissions: 67K
 * Testcase Example:  '3'
 *
 * 给出 n 代表生成括号的对数，请你写出一个函数，使其能够生成所有可能的并且有效的括号组合。
 *
 * 例如，给出 n = 3，生成结果为：
 *
 * [
 * ⁠ "((()))",
 * ⁠ "(()())",
 * ⁠ "(())()",
 * ⁠ "()(())",
 * ⁠ "()()()"
 * ]
 *
 *
 */

// @lc code=start
/**
 * @param {number} n
 * @return {string[]}
 */
var generateParenthesis = function(n) {
  const ctx = {list: []};
  gen(ctx, 0, 0, n, '');
  return ctx.list;
};

function gen(ctx, left, right, n, result) {
  if (left === n && right === n) {
    ctx.list.push(result);
    return;
  }
  if (left < n) {
    gen(ctx, left + 1, right, n, result + '(');
  }
  if (right < left && right < n) {
    gen(ctx, left, right + 1, n, result + ')');
  }
}
// @lc code=end
