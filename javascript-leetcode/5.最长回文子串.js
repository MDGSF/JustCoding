/*
 * @lc app=leetcode.cn id=5 lang=javascript
 *
 * [5] 最长回文子串
 *
 * https://leetcode-cn.com/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (27.40%)
 * Likes:    1373
 * Dislikes: 0
 * Total Accepted:    125.8K
 * Total Submissions: 458.2K
 * Testcase Example:  '"babad"'
 *
 * 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
 *
 * 示例 1：
 *
 * 输入: "babad"
 * 输出: "bab"
 * 注意: "aba" 也是一个有效答案。
 *
 *
 * 示例 2：
 *
 * 输入: "cbbd"
 * 输出: "bb"
 *
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @return {string}
 */
var longestPalindrome = function(s) {
  /*
   https://www.geeksforgeeks.org/manachers-algorithm-linear-time-longest-palindromic-substring-part-1/

  ------------------------------------------------------------
   centerLeft  curLeft  centerPosition  curRight  centerRight
  ------------------------------------------------------------

                  "babadada"
                  |   b   |   a   |   b   |   a   |   d   |   a   |   d   |   a   |
                  0   1   0   3   0   3   0   1   0   3   0   5   0   3   0   1   0

position Index    0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16
string Index          0       1       2       3       4       5       6       7
   */
  if (s.length < 2) {
    return s;
  }

  const N = 2 * s.length + 1;
  const L = new Array(N);
  L[0] = 0;
  L[1] = 1;
  let centerPosition = 1;
  let centerRight = 2;

  let maxLPSLength = 0;
  let maxLPSCenterPosition = 0;

  for (let i = 2; i < N; i += 1) {
    let needExpand = false;
    let curRight = i;
    let curLeft = 2 * centerPosition - curRight;

    let diff = centerRight - curRight;
    if (diff > 0) {
      if (L[curLeft] < diff) {
        L[curRight] = L[curLeft];
      } else if (L[curLeft] === diff && curRight === N - 1) {
        L[curRight] = L[curLeft];
      } else if (L[curLeft] === diff && curRight < N - 1) {
        L[curRight] = L[curLeft];
        needExpand = true;
      } else if (L[curLeft] > diff) {
        L[curRight] = diff;
        needExpand = true;
      }
    } else {
      L[curRight] = 0;
      needExpand = true;
    }

    if (needExpand) {
      while (
        i + L[i] < N &&
        i - L[i] > 0 &&
        ((i + L[i] + 1) % 2 == 0 ||
          (Math.floor((i + L[i] + 1) / 2) < s.length &&
            s[Math.floor((i + L[i] + 1) / 2)] ==
              s[Math.floor((i - L[i] - 1) / 2)]))
      ) {
        L[i]++;
      }
    }

    if (L[curRight] > maxLPSLength) {
      maxLPSLength = L[curRight];
      maxLPSCenterPosition = curRight;
    }

    if (curRight + L[curRight] >= centerRight) {
      centerPosition = curRight;
      centerRight = curRight + L[curRight];
    }
  }

  const start = Math.floor((maxLPSCenterPosition - maxLPSLength) / 2);
  const end = start + maxLPSLength;
  return s.substring(start, end);
};

// @lc code=end
