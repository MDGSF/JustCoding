
/**
 * @param {string} pattern
 * @param {string} str
 * @return {boolean}
 */
var wordPattern = function (pattern, str) {
  let words = str.split(' ');
  if (pattern.length != words.length) {
    return false;
  }
  
  const patternMap = new Map();
  const wordsMap = new Map();
  for (let i = 0; i < pattern.length; i += 1) {
    const patternValue = patternMap.get(pattern[i]);
    const wordsValue = wordsMap.get(words[i]);
    
    if (!patternValue) {
      patternMap.set(pattern[i], words[i]);
    } else if (patternValue !== words[i]) {
      return false;
    }
    
    if (!wordsValue) {
      wordsMap.set(words[i], pattern[i]);
    } else if (wordsValue !== pattern[i]) {
      return false;
    }
  }
  return true;
};
