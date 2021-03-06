/*
 * @lc app=leetcode.cn id=69 lang=javascript
 *
 * [69] x 的平方根
 *
 * https://leetcode-cn.com/problems/sqrtx/description/
 *
 * algorithms
 * Easy (37.06%)
 * Likes:    242
 * Dislikes: 0
 * Total Accepted:    66.3K
 * Total Submissions: 178.7K
 * Testcase Example:  '4'
 *
 * 实现 int sqrt(int x) 函数。
 *
 * 计算并返回 x 的平方根，其中 x 是非负整数。
 *
 * 由于返回类型是整数，结果只保留整数的部分，小数部分将被舍去。
 *
 * 示例 1:
 *
 * 输入: 4
 * 输出: 2
 *
 *
 * 示例 2:
 *
 * 输入: 8
 * 输出: 2
 * 说明: 8 的平方根是 2.82842...,
 * 由于返回类型是整数，小数部分将被舍去。
 *
 *
 */

// @lc code=start
/**
 * @param {number} x
 * @return {number}
 */
var mySqrt = function(x) {
  if (x === 0 || x === 1) {
    return x;
  }
  let l = 1;
  let r = x;
  let res = null;
  while (l <= r) {
    let m = Math.floor((l + r) / 2);
    if (m * m === x) {
      return m;
    } else if (m * m > x) {
      r = m - 1;
    } else {
      l = m + 1;
      res = m;
    }
  }
  return res;
};
// @lc code=end
