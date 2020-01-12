/*
 * @lc app=leetcode.cn id=91 lang=javascript
 *
 * [91] 解码方法
 *
 * https://leetcode-cn.com/problems/decode-ways/description/
 *
 * algorithms
 * Medium (21.89%)
 * Likes:    212
 * Dislikes: 0
 * Total Accepted:    19.5K
 * Total Submissions: 89.1K
 * Testcase Example:  '"12"'
 *
 * 一条包含字母 A-Z 的消息通过以下方式进行了编码：
 *
 * 'A' -> 1
 * 'B' -> 2
 * ...
 * 'Z' -> 26
 *
 *
 * 给定一个只包含数字的非空字符串，请计算解码方法的总数。
 *
 * 示例 1:
 *
 * 输入: "12"
 * 输出: 2
 * 解释: 它可以解码为 "AB"（1 2）或者 "L"（12）。
 *
 *
 * 示例 2:
 *
 * 输入: "226"
 * 输出: 3
 * 解释: 它可以解码为 "BZ" (2 26), "VF" (22 6), 或者 "BBF" (2 2 6) 。
 *
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var numDecodings = function(s) {
  /*
  dp[i] 表示从 0 到 i (包含i) 的字符串能解析的可能性数目。
  dp[i] = dp[i-1] + dp[i-2]
  */
  if (s === null || s.length === 0) { return 0; }
  let dp = new Array(s.length).fill(0);
  dp[0] = s.charAt(0) !== '0' ? 1 : 0;
  for (let i = 1; i < s.length; i += 1) {
    const first = +s.charAt(i);
    const second = +s.substring(i-1, i+1);
    if (first >= 1 && first <= 9) {
      dp[i] += dp[i-1];
    }
    if (second >= 10 && second <= 26) {
      dp[i] += i >= 2 ? dp[i-2] : 1;
    }
  }
  return dp[s.length-1];
};

// @lc code=end

const s = "01"
const result = numDecodings(s);
console.log(result);

