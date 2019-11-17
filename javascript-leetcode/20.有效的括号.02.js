/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
  let stack = [];
  let m = {')': '(', ']': '[', '}': '{'};
  for (let c of s) {
    if (!m.hasOwnProperty(c)) {
      stack.push(c);
    } else if (stack.length === 0 || m[c] !== stack.pop()) {
      return false;
    }
  }
  return stack.length === 0;
};

const result = isValid('()');
console.log(result);
