/*
 * @lc app=leetcode.cn id=67 lang=javascript
 *
 * [67] 二进制求和
 *
 * https://leetcode-cn.com/problems/add-binary/description/
 *
 * algorithms
 * Easy (50.99%)
 * Likes:    251
 * Dislikes: 0
 * Total Accepted:    44.6K
 * Total Submissions: 87.4K
 * Testcase Example:  '"11"\n"1"'
 *
 * 给定两个二进制字符串，返回他们的和（用二进制表示）。
 *
 * 输入为非空字符串且只包含数字 1 和 0。
 *
 * 示例 1:
 *
 * 输入: a = "11", b = "1"
 * 输出: "100"
 *
 * 示例 2:
 *
 * 输入: a = "1010", b = "1011"
 * 输出: "10101"
 *
 */

// @lc code=start
/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
var addBinary = function(a, b) {
  let result = [];
  let carry = 0;
  let i = a.length - 1;
  let j = b.length - 1;
  while (i >= 0 || j >= 0) {
    let cur = carry;
    cur += i >= 0 ? +a.charAt(i) : 0;
    cur += j >= 0 ? +b.charAt(j) : 0;
    result.push(cur % 2);
    carry = Math.floor(cur / 2);
    i -= 1;
    j -= 1;
  }
  result.push(carry === 1 ? carry : "");
  return result.reverse().join("");
};
// @lc code=end

const result = addBinary("1010", "1011");
console.log(result);

