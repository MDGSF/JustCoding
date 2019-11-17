/**
 * @param {number[]} cost
 * @return {number}
 */
var minCostClimbingStairs = function(cost) {
  let f0 = cost[0];
  let f1 = cost[1];
  let f2 = cost[2] + Math.min(f0, f1);
  for (let i = 3; i < cost.length; i += 1) {
    [f0, f1, f2] = [f1, f2, cost[i] + Math.min(f1, f2)];
  }
  return Math.min(f1, f2);
};
