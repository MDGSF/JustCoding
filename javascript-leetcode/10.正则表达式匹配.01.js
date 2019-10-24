/*
 * @lc app=leetcode.cn id=10 lang=javascript
 *
 * [10] 正则表达式匹配
 *
 * https://leetcode-cn.com/problems/regular-expression-matching/description/
 *
 * algorithms
 * Hard (25.05%)
 * Likes:    713
 * Dislikes: 0
 * Total Accepted:    35.2K
 * Total Submissions: 139.9K
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
 *
 * '.' 匹配任意单个字符
 * '*' 匹配零个或多个前面的那一个元素
 *
 *
 * 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
 *
 * 说明:
 *
 *
 * s 可能为空，且只包含从 a-z 的小写字母。
 * p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。
 *
 *
 * 示例 1:
 *
 * 输入:
 * s = "aa"
 * p = "a"
 * 输出: false
 * 解释: "a" 无法匹配 "aa" 整个字符串。
 *
 *
 * 示例 2:
 *
 * 输入:
 * s = "aa"
 * p = "a*"
 * 输出: true
 * 解释: 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
 *
 *
 * 示例 3:
 *
 * 输入:
 * s = "ab"
 * p = ".*"
 * 输出: true
 * 解释: ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
 *
 *
 * 示例 4:
 *
 * 输入:
 * s = "aab"
 * p = "c*a*b"
 * 输出: true
 * 解释: 因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。
 *
 *
 * 示例 5:
 *
 * 输入:
 * s = "mississippi"
 * p = "mis*is*p*."
 * 输出: false
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
var isMatch = function(s, p) {
  const ZERO_OR_MORE_CHARS = '*';
  const ANY_CHAR = '.';

  const matchMatrix = Array(s.length + 1)
    .fill(null)
    .map(() => {
      return Array(p.length + 1).fill(null);
    });

  matchMatrix[0][0] = true;

  for (let columnIndex = 1; columnIndex <= p.length; columnIndex += 1) {
    const patternIndex = columnIndex - 1;
    if (p[patternIndex] === ZERO_OR_MORE_CHARS) {
      matchMatrix[0][columnIndex] = matchMatrix[0][columnIndex - 2];
    } else {
      matchMatrix[0][columnIndex] = false;
    }
  }

  for (let rowIndex = 1; rowIndex <= s.length; rowIndex += 1) {
    matchMatrix[rowIndex][0] = false;
  }

  for (let rowIndex = 1; rowIndex <= s.length; rowIndex += 1) {
    for (let columnIndex = 1; columnIndex <= p.length; columnIndex += 1) {
      const stringIndex = rowIndex - 1;
      const patternIndex = columnIndex - 1;

      if (p[patternIndex] === ZERO_OR_MORE_CHARS) {
        if (matchMatrix[rowIndex][columnIndex - 2] === true) {
          matchMatrix[rowIndex][columnIndex] = true;
        } else if (
          matchMatrix[rowIndex - 1][columnIndex] === true &&
          (p[patternIndex - 1] === s[stringIndex] ||
            p[patternIndex - 1] === ANY_CHAR)
        ) {
          matchMatrix[rowIndex][columnIndex] = true;
        } else {
          matchMatrix[rowIndex][columnIndex] = false;
        }
      } else if (
        p[patternIndex] === s[stringIndex] ||
        p[patternIndex] === ANY_CHAR
      ) {
        matchMatrix[rowIndex][columnIndex] =
          matchMatrix[rowIndex - 1][columnIndex - 1];
      } else {
        matchMatrix[rowIndex][columnIndex] = false;
      }
    }
  }

  return matchMatrix[s.length][p.length];
};
// @lc code=end
