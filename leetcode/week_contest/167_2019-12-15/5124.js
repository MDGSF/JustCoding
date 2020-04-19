/**
 * @param {number} low
 * @param {number} high
 * @return {number[]}
 */
var sequentialDigits = function(low, high) {
  const m = {};
  for (let i = 1; i < 10; i += 1) {
    let ta = [];
    for (let j = 1; j <= 10 - i; j += 1) {
      let num = j;
      let tj = j + 1;
      for (let k = 0; k < i - 1; k += 1) {
        num = num * 10 + tj;
        tj += 1;
      }
      ta.push(num);
    }
    m[i] = ta;
  }
  let lowCount = count(low);
  let highCount = count(high);
  let curIndex = lowCount;
  let result = [];
  let finished = false;
  while (!finished && curIndex < 10) {
    let ta = m[curIndex];
    for (let i = 0; i < ta.length; i += 1) {
      if (ta[i] >= low && ta[i] <= high) {
        result.push(ta[i]);
      } else if (ta[i] > high) {
        finished = true
        break;
      }
    }
    curIndex += 1;
  }
  return result;
};

function count(num) {
  let count = 0;
  while (num > 0) {
    num = Math.floor(num / 10);
    count += 1;
  }
  return count;
}

console.log(sequentialDigits(10, 1000000000));
//console.log(sequentialDigits(1000, 13000));
