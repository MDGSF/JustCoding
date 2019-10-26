const hasOwnProperty = Object.prototype.hasOwnProperty;

class TrieNode {
  constructor(c, isCompleteWord = false) {
    this.c = c;
    this.isCompleteWord = isCompleteWord;
    this.children = {};
  }
  
  hasChildren() {
    return Object.keys(this.children).length !== 0;
  }
  
  addChild(c, isCompleteWord = false) {
    if (!hasOwnProperty.call(this.children, c)) {
      this.children[c] = new TrieNode(c, isCompleteWord);
    }
    let childNode = this.children[c];
    childNode.isCompleteWord = childNode.isCompleteWord || isCompleteWord;
    return childNode;
  }
  
  getChild(c) {
    if (!hasOwnProperty.call(this.children, c)) {
      return null;
    }
    return this.children[c];
  }
}

/**
 * Initialize your data structure here.
 */
var Trie = function() {
  this.head = new TrieNode('*');
};

/**
 * Inserts a word into the trie. 
 * @param {string} word
 * @return {void}
 */
Trie.prototype.insert = function(word) {
  let cur = this.head;   
  for (let i = 0; i < word.length; i += 1) {
    const isCompleteWord = i === word.length - 1;
    cur = cur.addChild(word[i], isCompleteWord);
  }
};

/**
 * Returns if the word is in the trie. 
 * @param {string} word
 * @return {boolean}
 */
Trie.prototype.search = function(word) {
  let cur = this.head;    
  for (let i = 0; i < word.length; i += 1) {
    cur = cur.getChild(word[i]);
    if (cur === null) {
      return false;
    }
    if (i === word.length - 1) {
      return cur.isCompleteWord;
    }
  }
  return false;
};

/**
 * Returns if there is any word in the trie that starts with the given prefix. 
 * @param {string} prefix
 * @return {boolean}
 */
Trie.prototype.startsWith = function(prefix) {
  let cur = this.head;    
  for (let i = 0; i < prefix.length; i += 1) {
    cur = cur.getChild(prefix[i]);
    if (cur === null) {
      return false;
    }
    if (i === prefix.length - 1) {
      return cur.hasChildren() || cur.isCompleteWord;
    }
  }
  return false;
};

/** 
 * Your Trie object will be instantiated and called as such:
 * var obj = new Trie()
 * obj.insert(word)
 * var param_2 = obj.search(word)
 * var param_3 = obj.startsWith(prefix)
 */