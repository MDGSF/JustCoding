/**
 * @param {number[][]} grid
 * @param {number} k
 * @return {number}
 */
var shortestPath = function(grid, k) {
  let dx = [1, -1, 0, 0];
  let dy = [0, 0, 1, -1];
  let m = grid.length;
  let n = grid[0].length;
  let visited = new Array(m).fill(null).map(() => {
    return new Array(n).fill(null).map(() => {
      return new Array(k + 1).fill(false);
    });
  });

  let queue = [];
  queue.push({row: 0, col: 0, k: k, path: 0});
  while (queue.length > 0) {
    let curNode = queue.shift();
    if (visited[curNode.row][curNode.col][curNode.k]) {
      continue;
    }
    visited[curNode.row][curNode.col][curNode.k] = true;

    // process curNode
    if (curNode.row === m - 1 && curNode.col === n - 1) {
      return curNode.path;
    }

    // nodes = generae_releated_nodes(curNode)
    for (let i = 0; i < 4; i += 1) {
      const newRow = curNode.row + dx[i];
      const newCol = curNode.col + dy[i];
      if (newRow < 0 || newCol < 0 || newRow >= m || newCol >= n) {
        continue;
      }

      const newPath = curNode.path + 1;
      let newk = curNode.k;
      if (grid[newRow][newCol] === 1) {
        if (newk > 0) {
          newk -= 1;
        } else {
          continue;
        }
      }

      if (visited[newRow][newCol][newk]) {
        continue;
      }
      queue.push({row: newRow, col: newCol, k: newk, path: newPath});
    }
  }
  return -1;
};

//grid =
//[[0,0,0],
// [1,1,0],
// [0,0,0],
// [0,1,1],
// [0,0,0]];
//k = 1;

//grid =
//[[0,1,1],
// [1,1,1],
// [1,0,0]];
//k = 1;

grid = [[0,0,1,0,0,0,0,1,0,1,1,0,0,1,1],[0,0,0,1,1,0,0,1,1,0,1,0,0,0,1],[1,1,0,0,0,0,0,1,0,1,0,0,1,0,0],[1,0,1,1,1,1,0,0,1,1,0,1,0,0,1],[1,0,0,0,1,1,0,1,1,0,0,1,1,1,1],[0,0,0,1,1,1,0,1,1,0,0,1,1,1,1],[0,0,0,1,0,1,0,0,0,0,1,1,0,1,1],[1,0,0,1,1,1,1,1,1,0,0,0,1,1,0],[0,0,1,0,0,1,1,1,1,1,0,1,0,0,0],[0,0,0,1,1,0,0,1,1,1,1,1,1,0,0],[0,0,0,0,1,1,1,0,0,1,1,1,0,1,0]];
k = 27;

const result = shortestPath(grid, k);
console.log(result);
