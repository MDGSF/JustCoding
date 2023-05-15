/*
dp[i][2]

dp[i][0] 不做第i个任务
dp[i][1] 做第i个任务

dp[i][0] = max(dp[i-1][0], dp[i-1][1])
dp[i][1] = max(dp[i-1][0], dp[i-1][1]) + array_w[i]
*/
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
  int T; // 工作时长
  int n; // 工作数量
  std::cin >> T >> n;

  std::vector<int> array_t(n, 0);
  std::vector<int> array_w(n, 0);

  for (int i = 0; i < n; ++i) {
    int t; // 消耗时长
    int w; // 报酬
    std::cin >> t >> w;
    array_t[i] = t;
    array_w[i] = w;
  }

  std::vector<std::vector<int>> dp_value(n);
  for (int i = 0; i < n; ++i) {
    std::vector<int> col(2, 0);
    dp_value[i] = col;
  }

  std::vector<std::vector<int>> dp_times(n);
  for (int i = 0; i < n; ++i) {
    std::vector<int> col(2, 0);
    dp_times[i] = col;
  }

  // 不做第0个任务
  dp_value[0][0] = 0;
  dp_times[0][0] = T;

  // 做第0个任务
  if (array_t[0] <= T) {
    dp_value[0][1] = array_w[0];
    dp_times[0][1] = T - array_t[0];
  }

  for (int i = 1; i < n; ++i) {
    // 不做第i个任务
    if (dp_value[i-1][0] >= dp_value[i-1][1]) {
      dp_value[i][0] = dp_value[i-1][0];
      dp_times[i][0] = dp_times[i-1][0];
    } else {
      dp_value[i][0] = dp_value[i-1][1];
      dp_times[i][0] = dp_times[i-1][1];
    }

    // 做第i个任务
    // 情况一：从[i-1][0]过来
    int value0 = 0;
    int times0 = 0;
    if (dp_times[i-1][0] >= array_t[i]) {
      value0 = dp_value[i-1][0] + array_w[i];
      times0 = dp_times[i-1][0] - array_t[i];
    }
    // 情况二：从[i-1][1]过来
    int value1 = 0;
    int times1 = 0;
    if (dp_times[i-1][1] >= array_t[i]) {
      value1 = dp_value[i-1][1] + array_w[i];
      times1 = dp_times[i-1][1] - array_t[i];
    }

    if (value0 >= value1) {
      dp_value[i][1] = value0;
      dp_times[i][1] = times0;
    } else {
      dp_value[i][1] = value1;
      dp_times[i][1] = times1;
    }
  }

  int max_value = std::max(dp_value[n-1][0], dp_value[n-1][1]);
  std::cout << max_value << std::endl;

  return 0;
}
