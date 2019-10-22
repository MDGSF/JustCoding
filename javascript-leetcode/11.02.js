/*
双指针法：
一个指针指向最左边，
一个指针指向最右边，
哪个高度更低，哪个就朝中间移动。
*/

/**
 * @param {number[]} height
 * @return {number}
 */
var maxArea = function(height) {
  let l = 0;
  let r = height.length - 1;
  let maxarea = 0;
  while (l < r) {
    const h = min(height[l], height[r]);
    const w = r - l;
    const curarea = h * w;
    if (curarea > maxarea) {
      maxarea = curarea;
    }

    if (height[l] < height[r]) {
      l += 1;
    } else {
      r -= 1;
    }
  }
  return maxarea;
};

function min(a, b) {
  return a < b ? a : b;
}
