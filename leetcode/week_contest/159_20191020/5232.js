function isSuperObj(obj1, obj2) {
  const keys1 = Object.keys(obj1);
  const keys2 = Object.keys(obj2);
  for (let i = 0; i < keys2.length; i += 1) {
    const key = keys1[i];
    if (!hasOwnProperty.call(obj1, key)) {
      return false;
    }
    if (obj1[key] < obj2[key]) {
      return false;
    }
  }
  return true;
}

/**
 * @param {string} s
 * @return {number}
 */
var balancedString = function(s) {
  const hasOwnProperty = Object.prototype.hasOwnProperty;
  const n = s.length / 4;
  const m = {};

  for (let i = 0; i < s.length; i += 1) {
    const c = s[i];
    if (!hasOwnProperty.call(m, c)) {
      m[c] = 1;
    } else {
      m[c] += 1;
    }
  }

  const keys = Object.keys(m);
  for (let i = 0; i < keys.length; i += 1) {
    const key = keys[i];
    if (m[key] <= n) {
      delete m[key];
    } else {
      m[key] -= n;
    }
  }

  if (Object.keys(m).length === 0) {
    return 0;
  }

  const subm = {};
  let l = 0;
  let r = 0;
  let minlength = 0;
  subm[s[0]] = 1;

  while (true) {
    if (isSuperObj(subm, m)) {
      const curLength = r - l + 1;
      if (minlength === 0) {
        minlength = curLength;
      } else if (curLength < minlength) {
        minlength = curLength;
      }

      const c = s[l];
      if (subm[c] === 1) {
        delete subm[c];
      } else {
        subm[c] -= 1;
      }
      l += 1;
    } else {
      r += 1;
      if (r >= s.length) {
        break;
      }
      const c = s[r];
      if (!hasOwnProperty.call(subm, c)) {
        subm[c] = 1;
      } else {
        subm[c] += 1;
      }
    }
  }

  return minlength;
};

//ret = balancedString('qqer');
//ret = balancedString('qqwe');
//ret = balancedString('qqqw');
//ret = balancedString('qqqq');
//ret = balancedString('WWEQERQWQWWRWWERQWEQ');
console.log(ret);
