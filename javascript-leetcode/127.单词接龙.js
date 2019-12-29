/**
 * @param {string} beginWord
 * @param {string} endWord
 * @param {string[]} wordList
 * @return {number}
 */
var ladderLength = function(beginWord, endWord, wordList) {
  let wordSet = new Set(wordList);
  if (!wordSet.has(endWord)) { return false; }
  let beginSet = new Set();
  let endSet = new Set();
  let visited = new Set();
  let level = 1;
  beginSet.add(beginWord);
  endSet.add(endWord);
  while (beginSet.size > 0 && endSet.size > 0) {
    if (beginSet.size > endSet.size) {
      [beginSet, endSet] = [endSet, beginSet];
    }
    let temp = new Set();
    for (let word of beginSet) {
      let wordCharArray = word.split("");
      for (let i = 0; i < wordCharArray.length; i += 1) {
        let old = wordCharArray[i];
        for (let j = 0; j < 26; j += 1) {
          wordCharArray[i] = String.fromCharCode(97+j);
          let next = wordCharArray.join("");
          if (endSet.has(next)) {
            return level + 1;
          }
          if (!visited.has(next) && wordSet.has(next)) {
            visited.add(next);
            temp.add(next);
          }
        }
        wordCharArray[i] = old;
      }
    }
    beginSet = temp;
    level += 1;
  }
  return 0;
};

const beginWord = "hit"
const endWord = "cog"
const wordList = ["hot","dot","dog","lot","log","cog"]

const result = ladderLength(beginWord, endWord, wordList);
console.log(result);

