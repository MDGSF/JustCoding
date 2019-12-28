/**
 * @param {string} digits
 * @return {string[]}
 */

const m = {
  '2': ['a', 'b', 'c'],
  '3': ['d', 'e', 'f'],
  '4': ['g', 'h', 'i'],
  '5': ['j', 'k', 'l'],
  '6': ['m', 'n', 'o'],
  '7': ['p', 'q', 'r', 's'],
  '8': ['t', 'u', 'v'],
  '9': ['w', 'x', 'y', 'z'],
};

var letterCombinations = function(digits) {
  if (digits.length === 0) { return []; }
  let result = [];
  dfs(result, "", digits);
  return result;
};

function dfs(result, node, digits) {
  if (digits.length === 0) {
    result.push(node);
    return;
  }
  let letters = m[digits[0]];
  for (let i = 0; i < letters.length; i += 1) {
    let letter = letters[i];
    dfs(result, node + letter, digits.substring(1));
  }
}
