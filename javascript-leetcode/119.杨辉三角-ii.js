/*
 * @lc app=leetcode.cn id=119 lang=javascript
 *
 * [119] 杨辉三角 II
 *
 * https://leetcode-cn.com/problems/pascals-triangle-ii/description/
 *
 * algorithms
 * Easy (54.10%)
 * Likes:    88
 * Dislikes: 0
 * Total Accepted:    27K
 * Total Submissions: 46.1K
 * Testcase Example:  '3'
 *
 * 给定一个非负索引 k，其中 k ≤ 33，返回杨辉三角的第 k 行。
 *
 *
 *
 * 在杨辉三角中，每个数是它左上方和右上方的数的和。
 *
 * 示例:
 *
 * 输入: 3
 * 输出: [1,3,3,1]
 *
 *
 * 进阶：
 *
 * 你可以优化你的算法到 O(k) 空间复杂度吗？
 *
 */

// @lc code=start
/**
 * @param {number} rowIndex
 * @return {number[]}
 */
var getRow = function(rowIndex) {
  if (rowIndex === 0) {
    return [1];
  }
  if (rowIndex === 1) {
    return [1, 1];
  }
  let prev = [1, 1];
  let cur = [];
  for (let i = 2; i <= rowIndex; i += 1) {
    cur[0] = 1;
    cur[i] = 1;
    for (let j = 1; j < i; j += 1) {
      cur[j] = prev[j - 1] + prev[j];
    }
    prev = cur;
    cur = [];
  }
  return prev;
};
// @lc code=end
