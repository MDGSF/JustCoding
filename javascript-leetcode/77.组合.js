/*
 * @lc app=leetcode.cn id=77 lang=javascript
 *
 * [77] 组合
 *
 * https://leetcode-cn.com/problems/combinations/description/
 *
 * algorithms
 * Medium (70.74%)
 * Likes:    171
 * Dislikes: 0
 * Total Accepted:    22.4K
 * Total Submissions: 31.5K
 * Testcase Example:  '4\n2'
 *
 * 给定两个整数 n 和 k，返回 1 ... n 中所有可能的 k 个数的组合。
 *
 * 示例:
 *
 * 输入: n = 4, k = 2
 * 输出:
 * [
 * ⁠ [2,4],
 * ⁠ [3,4],
 * ⁠ [2,3],
 * ⁠ [1,2],
 * ⁠ [1,3],
 * ⁠ [1,4],
 * ]
 *
 */

// @lc code=start
/**
 * @param {number} n
 * @param {number} k
 * @return {number[][]}
 */
var combine = function(n, k) {
  let result = [];
  recursion(n, k, 1, [], result);
  return result;
};

function recursion(n, k, first, cur, result) {
  if (cur.length === k) {
    result.push(cur.slice(0))
    return;
  }

  for (let i = first; i <= n; i += 1) {
    cur.push(i);
    recursion(n, k, i + 1, cur, result);
    cur.pop();
  }
}

const result = combine(4, 2);
console.log(result);

// @lc code=end

