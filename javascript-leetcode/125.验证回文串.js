/*
 * @lc app=leetcode.cn id=125 lang=javascript
 *
 * [125] 验证回文串
 *
 * https://leetcode-cn.com/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (38.22%)
 * Likes:    121
 * Dislikes: 0
 * Total Accepted:    58.3K
 * Total Submissions: 141.3K
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * 给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。
 *
 * 说明：本题中，我们将空字符串定义为有效的回文串。
 *
 * 示例 1:
 *
 * 输入: "A man, a plan, a canal: Panama"
 * 输出: true
 *
 *
 * 示例 2:
 *
 * 输入: "race a car"
 * 输出: false
 *
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @return {boolean}
 */
var isPalindrome = function(s) {
  const sArray = Array.from(s.toLowerCase()).filter(c => /[0-9a-z]/i.test(c));
  let start = 0;
  let end = sArray.length - 1;
  while (start < end) {
    if (sArray[start] !== sArray[end]) {
      return false;
    }
    start += 1;
    end -= 1;
  }
  return true;
};

const result = isPalindrome('A man, a plan, a canal: Panama');
console.log(result);
// @lc code=end
