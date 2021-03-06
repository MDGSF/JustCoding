/*
 * @lc app=leetcode.cn id=387 lang=javascript
 *
 * [387] 字符串中的第一个唯一字符
 *
 * https://leetcode-cn.com/problems/first-unique-character-in-a-string/description/
 *
 * algorithms
 * Easy (36.05%)
 * Likes:    150
 * Dislikes: 0
 * Total Accepted:    46.1K
 * Total Submissions: 112.2K
 * Testcase Example:  '"leetcode"'
 *
 * 给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1。
 *
 * 案例:
 *
 *
 * s = "leetcode"
 * 返回 0.
 *
 * s = "loveleetcode",
 * 返回 2.
 *
 *
 *
 *
 * 注意事项：您可以假定该字符串只包含小写字母。
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var firstUniqChar = function(s) {
  const m = new Map();
  for (let c of s) {
    if (m.has(c)) {
      m.set(c, m.get(c) + 1);
    } else {
      m.set(c, 1);
    }
  }
  for (let [key, value] of m) {
    if (value === 1) {
      return s.indexOf(key);
    }
  }
  return -1;
};
// @lc code=end

/*
function first(s) {
  const m = {};
  for (let c of s) {
    if (c in m) {
      m[c] += 1;
    } else {
      m[c] = 1;
    }
  }
  for (let i = 0; i < s.length; i += 1) {
    const c = s[i];
    if (m[c] === 1) {
      return i;
    }
  }
  return -1;
}
*/
