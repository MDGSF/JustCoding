/*
 * @lc app=leetcode.cn id=242 lang=javascript
 *
 * [242] 有效的字母异位词
 *
 * https://leetcode-cn.com/problems/valid-anagram/description/
 *
 * algorithms
 * Easy (56.04%)
 * Likes:    124
 * Dislikes: 0
 * Total Accepted:    55.6K
 * Total Submissions: 97.9K
 * Testcase Example:  '"anagram"\n"nagaram"'
 *
 * 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
 *
 * 示例 1:
 *
 * 输入: s = "anagram", t = "nagaram"
 * 输出: true
 *
 *
 * 示例 2:
 *
 * 输入: s = "rat", t = "car"
 * 输出: false
 *
 * 说明:
 * 你可以假设字符串只包含小写字母。
 *
 * 进阶:
 * 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isAnagram = function(s, t) {
  const m = {};
  for (let c of s) {
    if (c in m) {
      m[c] += 1;
    } else {
      m[c] = 1;
    }
  }
  for (let c of t) {
    if (c in m) {
      if (m[c] === 0) {
        return false;
      }
      m[c] -= 1;
    } else {
      return false;
    }
  }
  for (let key in m) {
    if (m[key] > 0) {
      return false;
    }
  }
  return true;
};
// @lc code=end
