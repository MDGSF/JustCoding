/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function(s) {
  let result = 0;
  let i = 0;
  let j = 0;
  while (j < s.length) {
    const t = s.substring(i, j).indexOf(s[j]);
    if (t === -1) {
      if (result < j - i + 1) {
        result = j - i + 1;
      } 
    } else {
      i = i + t + 1;
    }
    j += 1;
  }
  return result;
};