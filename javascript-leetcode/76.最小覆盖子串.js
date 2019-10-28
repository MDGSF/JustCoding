/*
 * @lc app=leetcode.cn id=76 lang=javascript
 *
 * [76] 最小覆盖子串
 *
 * https://leetcode-cn.com/problems/minimum-window-substring/description/
 *
 * algorithms
 * Hard (34.97%)
 * Likes:    243
 * Dislikes: 0
 * Total Accepted:    14.1K
 * Total Submissions: 40.7K
 * Testcase Example:  '"ADOBECODEBANC"\n"ABC"'
 *
 * 给你一个字符串 S、一个字符串 T，请在字符串 S 里面找出：包含 T 所有字母的最小子串。
 * 
 * 示例：
 * 
 * 输入: S = "ADOBECODEBANC", T = "ABC"
 * 输出: "BANC"
 * 
 * 说明：
 * 
 * 
 * 如果 S 中不存这样的子串，则返回空字符串 ""。
 * 如果 S 中存在这样的子串，我们保证它是唯一的答案。
 * 
 * 
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */
var minWindow = function(s, t) {
  const hasOwnProperty = Object.prototype.hasOwnProperty;
  let min = '';
  let left = 0;
  let right = -1;
  let m = {};

  t.split('').forEach(element => {
    if (!hasOwnProperty.call(m, element)) {
      m[element] = 1;
    } else {
      m[element] += 1;
    }
  });

  let count = Object.keys(m).length;

  while (right <= s.length) {
    if (count === 0) {
      // found a valid substring

      let current = s[left];

      if (hasOwnProperty.call(m, current)) {
        m[current] += 1;
      }

      if (m[current] > 0) {
        count += 1;
      }

      let temp = s.substring(left, right + 1);
      if (min === '') {
        min = temp;
      } else {
        min = min.length < temp.length ? min : temp;
      }

      left += 1;
    } else {
      right += 1;

      let current = s[right];

      if (hasOwnProperty.call(m, current)) {
        m[current] -= 1;
      }

      if (m[current] === 0) {
        count -= 1;
      }
    }
  }

  return min;
};
// @lc code=end

const result = minWindow("ADOBECODEBANC", "ABC");
console.log(result);
