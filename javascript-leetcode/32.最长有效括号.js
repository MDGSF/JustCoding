/*
 * @lc app=leetcode.cn id=32 lang=javascript
 *
 * [32] 最长有效括号
 *
 * https://leetcode-cn.com/problems/longest-valid-parentheses/description/
 *
 * algorithms
 * Hard (28.43%)
 * Likes:    363
 * Dislikes: 0
 * Total Accepted:    25K
 * Total Submissions: 88K
 * Testcase Example:  '"(()"'
 *
 * 给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。
 *
 * 示例 1:
 *
 * 输入: "(()"
 * 输出: 2
 * 解释: 最长有效括号子串为 "()"
 *
 *
 * 示例 2:
 *
 * 输入: ")()())"
 * 输出: 4
 * 解释: 最长有效括号子串为 "()()"
 *
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var longestValidParentheses = function(s) {
  /*
  dp[i] 为以下标 i 结尾的最长有效子字符串的长度。

  s[i] == ')' && s[i-1] == '('
    dp[i] = dp[i-2] + 2

  s[i] == ')' && s[i-1] == ')' && s[i - dp[i-1] - 1] == '('
    dp[i] = dp[i-1] + dp[i - dp[i-1] - 2] + 2
  */
  let result = 0;
  let dp = new Array(s.length).fill(0);
  for (let i = 1; i < s.length; i += 1) {
    if (s.charAt(i) === '(') {
      continue;
    }
    if (s.charAt(i-1) === '(') {
      dp[i] = (i >= 2 ? dp[i-2] : 0) + 2;
    } else if (i - dp[i-1] && s.charAt(i - dp[i-1] - 1) === '(') {
      dp[i] = dp[i-1] + (i - dp[i-1] >= 2 ? dp[i - dp[i-1] - 2] : 0) + 2;
    }
    result = Math.max(result, dp[i]);
  }
  return result;
};
// @lc code=end


const s = "(()"
const result = longestValidParentheses(s);
console.log(result);

