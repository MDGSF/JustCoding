/**
 * @param {string} s
 * @return {number}
 */
var balancedString = function(s) {
  const l = s.length / 4;
  const has = Object.prototype.hasOwnProperty;
  const m = {};
  for (let i = 0; i < s.length; i += 1) {
    const c = s[i];
    if (!has.call(m, c)) {
      m[c] = 1;
      continue;
    }
    m[c] += 1;
  }

  let result = 0;
  const keys = Object.keys(m);
  for (let i = 0; i < keys.length; i += 1) {
    const key = keys[i];
    const val = m[key];
    if (val > l) {
      result += val - l;
    }
  }
  return result;
};

//ret = balancedString('qqer');
//ret = balancedString('qqwe');
//ret = balancedString('qqqw');
//ret = balancedString('qqqq');
ret = balancedString('WWEQERQWQWWRWWERQWEQ');
console.log(ret);
