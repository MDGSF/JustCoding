/**
 * @param {character[][]} board
 * @param {string[]} words
 * @return {string[]}
 */
var findWords = function(board, words) {
  const trie = new Trie();
  for (let word of words) {
    trie.insert(word);
  }

  const visited = Array(board.length)
    .fill(null)
    .map(() => {
      return Array(board[0].length).fill(false);
    });

  const result = new Set();
  for (let i = 0; i < board.length; i += 1) {
    for (let j = 0; j < board[0].length; j += 1) {
      dfs(trie.head, board, i, j, '', result, visited);
    }
  }
  return Array.from(result);
};

const directions = [[1, 0], [-1, 0], [0, 1], [0, -1]];

function dfs(node, board, row, col, curStr, result, visited) {
  if (node.isCompleteWord) {
    result.add(curStr);
  }

  if (!canWalk(board, row, col, visited)) {
    return;
  }

  const c = board[row][col];
  if (!node.hasChild(c)) {
    return;
  }

  visited[row][col] = true;
  for (let dir of directions) {
    const newRow = row + dir[0];
    const newCol = col + dir[1];
    dfs(node.getChild(c), board, newRow, newCol, curStr + c, result, visited);
  }
  visited[row][col] = false;
}

function canWalk(board, row, col, visited) {
  return (
    row >= 0 &&
    row < board.length &&
    col >= 0 &&
    col < board[0].length &&
    !visited[row][col]
  );
}

class TrieNode {
  constructor(c, isCompleteWord = false) {
    this.c = c;
    this.isCompleteWord = isCompleteWord;
    this.children = new Map();
  }

  hasChild(c) {
    return this.children.has(c);
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

const words = ['oath', 'pea', 'eat', 'rain'];
const board = [
  ['o', 'a', 'a', 'n'],
  ['e', 't', 'a', 'e'],
  ['i', 'h', 'k', 'r'],
  ['i', 'f', 'l', 'v'],
];
const result = findWords(board, words);
console.log(result);
