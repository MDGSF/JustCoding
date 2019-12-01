/*
 * @lc app=leetcode.cn id=17 lang=javascript
 *
 * [17] 电话号码的字母组合
 *
 * https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (51.11%)
 * Likes:    478
 * Dislikes: 0
 * Total Accepted:    51.9K
 * Total Submissions: 101.4K
 * Testcase Example:  '"23"'
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
 *
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 *
 *
 *
 * 示例:
 *
 * 输入："23"
 * 输出：["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 *
 *
 * 说明:
 * 尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。
 *
 */

// @lc code=start
/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function(digits) {
  const m = {
    '2': ['a', 'b', 'c'],
    '3': ['d', 'e', 'f'],
    '4': ['g', 'h', 'i'],
    '5': ['j', 'k', 'l'],
    '6': ['m', 'n', 'o'],
    '7': ['p', 'q', 'r', 's'],
    '8': ['t', 'u', 'v'],
    '9': ['w', 'x', 'y', 'z'],
  };
  function combination(digits) {
    if (digits.length === 0) {
      return [];
    }
    if (digits.length === 1) {
      return m[digits];
    }
    let result = [];
    let sub = combination(digits.substring(1));
    let val = m[digits[0]];
    for (let i = 0; i < val.length; i += 1) {
      for (let j = 0; j < sub.length; j += 1) {
        result.push(val[i] + sub[j]);
      }
    }
    return result;
  }
  return combination(digits);
};

const result = letterCombinations('23');
console.log(result);

// @lc code=end
