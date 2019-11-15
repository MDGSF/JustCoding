/**
 * @param {number[]} nums
 * @return {number}
 */
var findShortestSubArray = function(nums) {
  const m = {};
  for (let i = 0; i < nums.length; i += 1) {
    const num = nums[i];
    if (num in m) {
      let obj = m[num];
      obj['right'] = i;
      obj['count'] += 1;
    } else {
      let obj = {};
      obj['left'] = i;
      obj['right'] = i;
      obj['count'] = 1;
      m[num] = obj;
    }
  }

  let maxdu = 0;
  for (let num in m) {
    let obj = m[num];
    maxdu = Math.max(maxdu, obj['count']);
  }

  let minLength = 0;
  for (let num in m) {
    let obj = m[num];
    if (obj['count'] === maxdu) {
      let curObjLength = obj['right'] - obj['left'];
      if (minLength === 0) {
        minLength = curObjLength;
      } else {
        minLength = Math.min(minLength, curObjLength);
      }
    }
  }

  return minLength + 1;
};

const result = findShortestSubArray([1, 2, 2, 3, 1]);
console.log(result);
