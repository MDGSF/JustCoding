/**
 * Initialize your data structure here.
 */
var WordDictionary = function() {
  this.len2Words = new Map();
};

/**
 * Adds a word into the data structure. 
 * @param {string} word
 * @return {void}
 */
WordDictionary.prototype.addWord = function(word) {
  const length = word.length;
  if (!this.len2Words.has(length)) {
    this.len2Words.set(length, new Set());
  }
  this.len2Words.get(length).add(word);
};

/**
 * Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. 
 * @param {string} word
 * @return {boolean}
 */
WordDictionary.prototype.search = function(word) {
  const length = word.length;
  if (!this.len2Words.has(length)) {
    return false;
  }
  const words = this.len2Words.get(length);
  for (let w of words) {
    if (isTheSame(w,word)) {
      return true;
    }
  }
  return false;
};

function isTheSame(word, pattern) {
  for (let i = 0; i < word.length; i += 1) {
    if (word[i] !== pattern[i] && pattern[i] !== '.') {
      return false;
    }
  }
  return true;
}

/** 
 * Your WordDictionary object will be instantiated and called as such:
 * var obj = new WordDictionary()
 * obj.addWord(word)
 * var param_2 = obj.search(word)
 */