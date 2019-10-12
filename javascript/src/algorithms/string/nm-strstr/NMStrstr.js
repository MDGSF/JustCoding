/*
@brief find word in text.
If word not exist in text, return -1.
If find word, return the index.
*/
function NMStrstr(text, word) {
  if (text.length === 0 || word.length === 0 || text.length < word.length) {
    return -1;
  }

  for (let i = 0; i < text.length; i += 1) {
    let success = true;
    let k = i;
    let j = 0;
    while (k < text.length && j < word.length) {
      if (text[k] !== word[j]) {
        success = false;
        break;
      }
      j += 1;
      k += 1;
    }
    if (success) {
      return i;
    }
  }
  return -1;
}

module.exports = NMStrstr;
