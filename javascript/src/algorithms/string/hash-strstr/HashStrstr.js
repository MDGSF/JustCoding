/*
@brief find word in text.
If word not exist in text, return -1.
If find word, return the index.
*/
function HashStrstr(text, word) {
  if (text.length === 0 || word.length === 0 || text.length < word.length) {
    return -1;
  }

  const wordkey = hash(word);
  const end = text.length - word.length;
  let iPrevKey = -1;

  for (let i = 0; i <= end; i += 1) {
    if (iPrevKey === -1) {
      const subs = text.slice(0, word.length);
      const subskey = hash(subs);
      if (subskey === wordkey) {
        return i;
      }
      iPrevKey = subskey;
    } else {
      const oldc = text[i - 1].charCodeAt(0);
      const newc = text[i + word.length - 1].charCodeAt(0);
      const subskey = (iPrevKey - oldc * 10 ** (word.length - 1)) * 10 + newc;
      if (subskey === wordkey) {
        return i;
      }
      iPrevKey = subskey;
    }
  }

  return -1;
}

function hash(s) {
  let key = 0;
  for (let i = 0; i < s.length; i += 1) {
    key = key * 10 + s[i].charCodeAt(0);
  }
  return key;
}

module.exports = HashStrstr;
