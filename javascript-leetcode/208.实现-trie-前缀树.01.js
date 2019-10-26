/*
 * @lc app=leetcode.cn id=208 lang=javascript
 *
 * [208] 实现 Trie (前缀树)
 *
 * https://leetcode-cn.com/problems/implement-trie-prefix-tree/description/
 *
 * algorithms
 * Medium (55.03%)
 * Likes:    139
 * Dislikes: 0
 * Total Accepted:    14.8K
 * Total Submissions: 23.9K
 * Testcase Example:  '["Trie","insert","search","search","startsWith","insert","search"]\n[[],["apple"],["apple"],["app"],["app"],["app"],["app"]]'
 *
 * 实现一个 Trie (前缀树)，包含 insert, search, 和 startsWith 这三个操作。
 *
 * 示例:
 *
 * Trie trie = new Trie();
 *
 * trie.insert("apple");
 * trie.search("apple");   // 返回 true
 * trie.search("app");     // 返回 false
 * trie.startsWith("app"); // 返回 true
 * trie.insert("app");
 * trie.search("app");     // 返回 true
 *
 * 说明:
 *
 *
 * 你可以假设所有的输入都是由小写字母 a-z 构成的。
 * 保证所有输入均为非空字符串。
 *
 *
 */

// @lc code=start
/**
 * Initialize your data structure here.
 */
var Trie = function() {
  this.head = new TrieNode(HEAD_CHARACTER);
};

/**
 * Inserts a word into the trie.
 * @param {string} word
 * @return {void}
 */
Trie.prototype.insert = function(word) {
  let cur = this.head;
  for (let i = 0; i < word.length; i += 1) {
    const isComplete = i === word.length - 1;
    cur = cur.addChild(word[i], isComplete);
  }
  return this;
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

// ---------------------------------------------------

const hasOwnProperty = Object.prototype.hasOwnProperty;
const HEAD_CHARACTER = '*';

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
 * Your Trie object will be instantiated and called as such:
 * var obj = new Trie()
 * obj.insert(word)
 * var param_2 = obj.search(word)
 * var param_3 = obj.startsWith(prefix)
 */
// @lc code=end
