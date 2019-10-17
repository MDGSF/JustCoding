/*
 * @lc app=leetcode.cn id=3 lang=javascript
 *
 * [3] 无重复字符的最长子串
 *
 * https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (31.65%)
 * Likes:    2539
 * Dislikes: 0
 * Total Accepted:    241.9K
 * Total Submissions: 762K
 * Testcase Example:  '"abcabcbb"'
 *
 * 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
 *
 * 示例 1:
 *
 * 输入: "abcabcbb"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
 *
 *
 * 示例 2:
 *
 * 输入: "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 *
 *
 * 示例 3:
 *
 * 输入: "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 *
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function(s) {
  const has = Object.prototype.hasOwnProperty;
  const m = {};
  let longestNum = 0;
  for (let i = 0; i < s.length; i += 1) {
    const c = s.charCodeAt(i);
    if (has.call(m, c)) {
      const keys = Object.keys(m);
      const keysNum = keys.length;
      if (keysNum > longestNum) {
        longestNum = keysNum;
      }

      const prevCharIndex = m[c];
      for (let j = 0; j < keysNum; j += 1) {
        const key = keys[j];
        const val = m[key];
        if (val <= prevCharIndex) {
          delete m[key];
        }
      }
    }
    m[c] = i;
  }
  const keysNum = Object.keys(m).length;
  if (keysNum > longestNum) {
    longestNum = keysNum;
  }
  return longestNum;
};
// @lc code=end
