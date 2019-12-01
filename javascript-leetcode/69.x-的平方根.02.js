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
  return MySqrt(x, 0.01)
};

function MySqrt(x, precision) {
  if (x === 0 || x === 1) {
    return x;
  }

  let l = 0;
  let r = x;
  while (l <= r) {
    let m = (l + r) / 2;
    let cur = m * m;
    if (cur - x >= -precision && cur - x <= precision) {
      return m;
    } else if (cur > x) {
      r = m;
    } else {
      l = m;
    }
  }
  return null;
}
// @lc code=end
