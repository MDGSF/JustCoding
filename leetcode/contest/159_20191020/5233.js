/**
 * @param {number[]} startTime
 * @param {number[]} endTime
 * @param {number[]} profit
 * @return {number}
 */
var jobScheduling = function(startTime, endTime, profit) {
  let maxProfit = [];
  maxProfit[0] = profit[0];
  let maxProfitJobs = new Set();
  maxProfitJobs.add(0);

  for (let i = 1; i < startTime.length; i += 1) {
    // 不做第 i 份工作
    const ret1 = maxProfit[i - 1];

    // 做第 i 份工作
    let ret2 = profit[i] + maxProfit[i - 1];
    let conflict = new Set();
    for (let j = 0; j < i; j += 1) {
      if (!maxProfitJobs.has(j)) {
        continue;
      }

      // 和第 i 份工作时间上冲突的工作就不能做了
      if (endTime[i] <= startTime[j] || endTime[j] <= startTime[i]) {
        // 时间没有冲突
      } else {
        ret2 -= profit[j];
        conflict.add(j);
      }
    }

    let newAdd = new Set();

    if (ret1 > ret2) {
      maxProfit[i] = ret1;
    } else {
      maxProfit[i] = ret2;
      maxProfitJobs.add(i);
      conflict.forEach(function(value) {
        maxProfitJobs.delete(value);
      });
    }
  }

  return maxProfit[startTime.length - 1];
};

//ret = jobScheduling([3, 2, 3, 3], [3, 4, 5, 6], [50, 10, 40, 70]);

//输入：startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit = [20,20,100,70,60]
//输出：150
ret = jobScheduling([1, 2, 3, 4, 6], [3, 5, 10, 6, 9], [20, 20, 100, 70, 60]);

console.log(ret);
