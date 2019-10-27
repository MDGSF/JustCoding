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
    
};
// @lc code=end

/*
var minWindowSlidingWindow = function (s, t) {
	// `right` is -1 since every loop, we start by expanding the right boundary
	// setting this to -1 ensures that we will check the first char on the first time
    let min = "", left = 0, right = -1;
    let map = {};
	
	// this creates a map for the characters we need to include in the substring
	// we store the character and its count since it can be repeated
	// for example: "BAAC"
    t.split('').forEach(element => {
        if (map[element]==null) map[element] = 1;
        else map[element] = map[element] + 1;
    });
	
	// sets how many different characters we still have
	// for example: given the input "BAAC", we still have 3 different characters need to check
    let count = Object.keys(map).length;

    while (right <= s.length) {
		// found a valid substring
        if (count == 0) {
		
			// try to shift left boudary to the right, this means the very left character will be removed
			// because of this, we need to check whats the affect by removing that character, 
            let current = s[left];
			
			// if this chacter is in our map, it means we ll need to find another one in the future
            if (map[current] != null) map[current]++;
			
			// * we must have the condition `>0` because for case like "BBBA...", count for B could be negative
            if (map[current] > 0) count++;    
			
            let temp = s.substring(left, right+1)
            if (min == "") min = temp;
            else min = min.length<temp.length?min:temp;
			
            left++;
        } else {
            right++;
            let current = s[right];
			
			// decrease the count for this character
            if (map[current] != null) map[current]--;
			
            if (map[current] == 0) count--;
        }
    }
    return min;
}
*/
