/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */
var minWindow = function(s, t) {
  const m = {};
  let left = 0;
  let right = -1;
  let minStr = '';
  
  t.split('').forEach(element => {
    if (element in m) {
      m[element] += 1;
    } else {
      m[element] = 1;
    }
  })
  
  let count = Object.keys(m).length;
  
  while (right <= s.length) {
    if (count === 0) {
      let temp = s.substring(left, right + 1);
      if (minStr === '') {
        minStr = temp;
      } else {
        minStr = minStr.length < temp.length ? minStr : temp;
      }
      
      let current = s[left];   
      if (current in m) {
        m[current] += 1;
      }
      if (m[current] > 0) {
        count += 1; 
      }
      left += 1;

    } else {
      right += 1;
      let current = s[right];
      if (current in m) {
        m[current] -= 1;
      }
      if (m[current] === 0) {
        count -= 1;
      }
    }
  }
  
  return minStr;
};