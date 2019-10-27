/*
 * @lc app=leetcode.cn id=212 lang=javascript
 *
 * [212] 单词搜索 II
 *
 * https://leetcode-cn.com/problems/word-search-ii/description/
 *
 * algorithms
 * Hard (34.69%)
 * Likes:    64
 * Dislikes: 0
 * Total Accepted:    5.3K
 * Total Submissions: 14.6K
 * Testcase Example:  '[["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]]\n["oath","pea","eat","rain"]'
 *
 * 给定一个二维网格 board 和一个字典中的单词列表 words，找出所有同时在二维网格和字典中出现的单词。
 * 
 * 
 * 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母在一个单词中不允许被重复使用。
 * 
 * 示例:
 * 
 * 输入: 
 * words = ["oath","pea","eat","rain"] and board =
 * [
 * ⁠ ['o','a','a','n'],
 * ⁠ ['e','t','a','e'],
 * ⁠ ['i','h','k','r'],
 * ⁠ ['i','f','l','v']
 * ]
 * 
 * 输出: ["eat","oath"]
 * 
 * 说明:
 * 你可以假设所有输入都由小写字母 a-z 组成。
 * 
 * 提示:
 * 
 * 
 * 你需要优化回溯算法以通过更大数据量的测试。你能否早点停止回溯？
 * 如果当前单词不存在于所有单词的前缀中，则可以立即停止回溯。什么样的数据结构可以有效地执行这样的操作？散列表是否可行？为什么？
 * 前缀树如何？如果你想学习如何实现一个基本的前缀树，请先查看这个问题： 实现Trie（前缀树）。
 * 
 * 
 */

class TrieNode {
  constructor(c, isCompleteWord = false) {
    this.c = c;
    this.isCompleteWord = isCompleteWord;
    this.children = new Map();
  }

  getChild(c) {
    if (!this.children.has(c)) {
      return null;
    }
    return this.children.get(c);
  }

  addChild(c, isCompleteWord) {
    if (!this.children.has(c)) {
      this.children.set(c, new TrieNode(c, isCompleteWord));
    }
    const childNode = this.children.get(c);
    childNode.isCompleteWord = childNode.isCompleteWord || isCompleteWord;
    return childNode;
  }

  hasChild(c) {
    return this.children.has(c);
  }
}

class Trie {
  constructor() {
    this.head = new TrieNode('*');
  }

  insert(word) {
    let cur = this.head;
    for (let i = 0; i < word.length; i += 1) {
      const isCompleteWord = i === word.length - 1;
      cur = cur.addChild(word[i], isCompleteWord);
    }
  }
}

// @lc code=start
/**
 * @param {character[][]} board
 * @param {string[]} words
 * @return {string[]}
 */
var findWords = function (board, words) {
  const trie = new Trie();
  for (let word of words) {
    trie.insert(word);
  }

  const result = new Set();
  for (let i = 0; i < board.length; i += 1) {
    for (let j = 0; j < board[0].length; j += 1) {
      findWordsInner(trie.head, board, i, j, '', result);
    }
  }
  return Array.from(result);
};

const EMPTY = '*';
const DIRS = [
  [0, 1],
  [1, 0],
  [-1, 0],
  [0, -1],
];

function findWordsInner(node, board, row, column, curStr, result) {
  if (node.isCompleteWord) {
    result.add(curStr);
  }

  if (!canWalk(board, row, column)) {
    return;
  }

  const c = board[row][column];
  if (!node.hasChild(c)) {
    return;
  }

  board[row][column] = EMPTY;
  for (let dir of DIRS) {
    const newRow = row + dir[0];
    const newColumn = column + dir[1];
    findWordsInner(node.getChild(c), board, newRow, newColumn, curStr + c, result);
  }
  board[row][column] = c;
}

function canWalk(board, row, column) {
  return row >= 0 &&
    row < board.length &&
    column >= 0 &&
    column < board[0].length &&
    board[row][column] !== EMPTY
}
// @lc code=end

