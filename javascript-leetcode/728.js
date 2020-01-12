/**
 * @param {number} left
 * @param {number} right
 * @return {number[]}
 */
var selfDividingNumbers = function(left, right) {
  let result = [];
  for (let i = left; i <= right; i += 1) {
    if (isValid(i)) {
      result.push(i);
    }
  }
  return result;
};

function isValid(number) {
  const s = ('' + number).split("");
  for (let i = 0; i < s.length; i += 1) {
    if (s[i] === '0' || number % +s[i] > 0) {
      return false;
    }
  }
  return true;
}

const result = selfDividingNumbers(1, 22);
console.log(result);

