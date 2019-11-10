/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
var maxSlidingWindow = function(nums, k) {
  const result = [];
  const deque = [];

  for (let i = 0; i < nums.length; i += 1) {
    if (i - k === deque[0]) {
      deque.shift();
    }

    for (let j = deque.length - 1; j >= 0; j -= 1) {
      if (nums[i] >= nums[deque[j]]) {
        deque.pop();
      }
    }

    deque.push(i);

    if (i >= k - 1) {
      result.push(nums[deque[0]]);
    }
  }

  return result;
};
