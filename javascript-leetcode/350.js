/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number[]}
 */
var intersect = function(nums1, nums2) {
  const s1 = new Map();
  const s2 = new Map();
  for (let i = 0; i < nums1.length; i += 1) {
    const num = nums1[i];
    if (s1.has(num)) {
      s1.set(num, s1.get(num) + 1);
    } else {
      s1.set(num, 1);
    }
  }
  for (let i = 0; i < nums2.length; i += 1) {
    const num = nums2[i];
    if (s2.has(num)) {
      s2.set(num, s2.get(num) + 1);
    } else {
      s2.set(num, 1);
    }
  }
  let result = [];
  for (let [key, value] of s1) {
    if (!s2.has(key)) {
      continue;
    }
    let minnum = Math.min(value, s2.get(key));
    for (let i = 0; i < minnum; i += 1) {
      result.push(key);
    }
  }
  return result;
};

//const nums1 = [1, 2, 2, 1];
//const nums2 = [2, 2];
const nums1 = [4, 9, 5];
const nums2 = [9, 4, 9, 8, 4];
const result = intersect(nums1, nums2);
console.log(result);
