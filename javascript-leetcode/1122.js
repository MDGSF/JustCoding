/**
 * @param {number[]} arr1
 * @param {number[]} arr2
 * @return {number[]}
 */
var relativeSortArray = function(arr1, arr2) {
  let m1 = new Map();
  let s2 = new Set(arr2);
  let temp = [];
  for (let i = 0; i < arr1.length; i += 1) {
    const num = arr1[i];
    if (m1.has(num)) {
      m1.set(num, m1.get(num) + 1);
    } else {
      m1.set(num, 1);
    }
    if (!s2.has(num)) {
      temp.push(num);
    }
  }
  temp.sort((a,b) => a - b);

  let result = [];
  for (let i = 0; i < arr2.length; i += 1) {
    const num = arr2[i];
    const val = m1.get(num);
    for (let i = 0; i < val; i += 1) {
      result.push(num);
    }
    m1.delete(num);
  }
  return result.concat(temp);
};

const arr1 = [2,3,1,3,2,4,6,7,9,2,19];
const arr2 = [2,1,4,3,9,6];
const result = relativeSortArray(arr1, arr2);
console.log(result);

