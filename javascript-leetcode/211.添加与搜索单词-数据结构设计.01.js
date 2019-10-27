/*
 * @lc app=leetcode.cn id=211 lang=javascript
 *
 * [211] 添加与搜索单词 - 数据结构设计
 *
 * https://leetcode-cn.com/problems/add-and-search-word-data-structure-design/description/
 *
 * algorithms
 * Medium (43.94%)
 * Likes:    68
 * Dislikes: 0
 * Total Accepted:    4.7K
 * Total Submissions: 11.8K
 * Testcase Example:  '["WordDictionary","addWord","addWord","addWord","search","search","search","search"]\n[[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]'
 *
 * 设计一个支持以下两种操作的数据结构：
 * 
 * void addWord(word)
 * bool search(word)
 * 
 * 
 * search(word) 可以搜索文字或正则表达式字符串，字符串只包含字母 . 或 a-z 。 . 可以表示任何一个字母。
 * 
 * 示例:
 * 
 * addWord("bad")
 * addWord("dad")
 * addWord("mad")
 * search("pad") -> false
 * search("bad") -> true
 * search(".ad") -> true
 * search("b..") -> true
 * 
 * 
 * 说明:
 * 
 * 你可以假设所有单词都是由小写字母 a-z 组成的。
 * 
 */

class TrieNode {
  constructor(c, isCompleteWord = false) {
    this.c = c;
    this.isCompleteWord = isCompleteWord;
    this.m = new Map();
  }

  hasChildren() {
    return this.m.size !== 0;
  }

  getChildren() {
    return Array.from(this.m.keys());
  }

  hasChild(c) {
    return this.m.has(c);
  }

  getChild(c) {
    if (!this.m.has(c)) {
      return null;
    }
    return this.m.get(c);
  }

  addChild(c, isCompleteWord) {
    if (!this.m.has(c)) {
      this.m.set(c, new TrieNode(c, isCompleteWord));
    }
    const childNode = this.m.get(c);
    childNode.isCompleteWord = childNode.isCompleteWord || isCompleteWord;
    return childNode;
  }
}

// @lc code=start
/**
 * Initialize your data structure here.
 */
var WordDictionary = function() {
  this.head = new TrieNode('*');
};

/**
 * Adds a word into the data structure. 
 * @param {string} word
 * @return {void}
 */
WordDictionary.prototype.addWord = function(word) {
  let cur = this.head;
  for (let i = 0; i < word.length; i += 1) {
    const isCompleteWord = i === word.length - 1;
    cur = cur.addChild(word[i], isCompleteWord);
  }
};

/**
 * Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. 
 * @param {string} word
 * @return {boolean}
 */
WordDictionary.prototype.search = function(word) {
  return this.searchInner(this.head, word);
};

WordDictionary.prototype.searchInner = function(node, word) {
  let cur = node;
  for (let i = 0; i < word.length; i += 1) {
    const c = word[i];
    if (c === '.') {
      const children = cur.getChildren();
      for (let child of children) {
        if (this.searchInner(cur.getChild(child), word.substring(i+1))) {
          return true;
        }
      }
      return false;
    } else {
      if (!cur.hasChild(c)) {
        return false;
      }
      cur = cur.getChild(c);
    }
  }
  return cur.isCompleteWord;
};

/** 
 * Your WordDictionary object will be instantiated and called as such:
 * var obj = new WordDictionary()
 * obj.addWord(word)
 * var param_2 = obj.search(word)
 */
// @lc code=end