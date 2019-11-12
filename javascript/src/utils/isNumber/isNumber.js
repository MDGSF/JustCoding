/*
数字 ascii码
 0     48
 1     49
 2     50
 3     51
 4     52
 5     53
 6     54
 7     55
 8     56
 9     57
*/

const isNumber = function(c) {
  const code = c.charCodeAt(0);
  if (code >= 48 && code <= 57) {
    return true;
  }
  return false;
};

module.exports = isNumber;
