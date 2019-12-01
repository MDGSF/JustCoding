/**
 * @param {number} n
 * @return {boolean}
 */
var isHappy = function(n) {
  const s = new Set();
  while (true) {
    const sum = calcSum(n);
    if (sum === 1) {
      return true;
    }
    if (s.has(sum)) {
      return false;
    }
    s.add(sum);
    n = sum;
  }
};

function calcSum(n) {
  let sum = 0;
  while (n > 0) {
    const cur = n % 10;
    sum += cur * cur;
    n = Math.floor(n / 10);
  }
  return sum;
}

const result = isHappy(19);
console.log(result);
