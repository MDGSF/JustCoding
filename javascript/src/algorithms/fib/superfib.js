function superfib(n, steps) {
  let states = new Array(steps.length).fill(null).map(() => {
    return new Array(n+1).fill(null);
  });

  for (let col = 1; col <= n; col += 1) {
    for (let row = 0; row < steps.length; row += 1) {
      let step = steps[row];
      let preCol = col - step;
      if (preCol < 0) {
        continue;
      }
      if (preCol === 0) {
        states[row][col] = 1;
        continue;
      }

      let preSum = 0;
      for (let preRow = 0; preRow < steps.length; preRow += 1) {
        if (preRow !== row && states[preRow][preCol] !== null) {
          preSum += states[preRow][preCol];
        }
      }
      if (preSum > 0) {
        states[row][col] = preSum;
      }
    }
  }

  let result = 0;
  for (let row = 0; row < steps.length; row += 1) {
    if (states[row][n] !== null) {
      result += states[row][n];
    }
  }
  return result;
}

const n = 5;
const steps = [1,2,3];
const result = superfib(n, steps);
console.log(result);


/*
steps = 1, 2
0
1 1
2 2
3 1+2, 2+1
4 1+2+1
5 2+1+2
   0  1  2  3  4  5
1  0  1  n  1  1  n
2  0  n  1  1  n  1


steps = 1, 2, 3
0
1 1
2 2
3 3, 1+2, 2+1
4 3+1, 1+2+1, 1+3
5 1+3+1, 3+2, 2+1+2, 2+3
   0  1  2  3  4  5
1  0  1  n  1  2  1
2  0  n  1  1  n  2
3  0  n  n  1  1  1
*/
