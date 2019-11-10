/*
 * @lc app=leetcode.cn id=42 lang=javascript
 *
 * [42] 接雨水
 *
 * https://leetcode-cn.com/problems/trapping-rain-water/description/
 *
 * algorithms
 * Hard (46.55%)
 * Likes:    632
 * Dislikes: 0
 * Total Accepted:    33.2K
 * Total Submissions: 71K
 * Testcase Example:  '[0,1,0,2,1,0,1,3,2,1,2,1]'
 *
 * 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
 *
 *
 *
 * 上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 感谢
 * Marcos 贡献此图。
 *
 * 示例:
 *
 * 输入: [0,1,0,2,1,0,1,3,2,1,2,1]
 * 输出: 6
 *
 */

// @lc code=start
/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function(height) {
  /*
 我们在遍历数组时维护一个栈。如果当前的条形块小于或等于栈顶的条形块，
 我们将条形块的索引入栈，意思是当前的条形块被栈中的前一个条形块界定。
 如果我们发现一个条形块长于栈顶，我们可以确定栈顶的条形块被当前条形块和栈的前一个条形块界定，
 因此我们可以弹出栈顶元素并且累加答案到 ans。
 和 84. 柱状图中最大的矩形 用栈的处理正好相反。
  */

  var peek = function(stack) {
    return stack[stack.length - 1];
  };

  let result = 0;
  let stack = [];
  let i = 0;
  for (let i = 0; i < height.length; i += 1) {
    while (stack.length > 0 && height[i] > height[peek(stack)]) {
      let top = stack.pop();
      if (stack.length === 0) {
        break;
      }
      let distance = i - peek(stack) - 1;
      let bounded_height =
        Math.min(height[i], height[peek(stack)]) - height[top];
      result += distance * bounded_height;
    }
    stack.push(i);
  }
  return result;
};
// @lc code=end
