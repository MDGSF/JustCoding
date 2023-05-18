#include <algorithm>
#include <iostream>
#include <vector>

void show(const std::vector<std::vector<int>>& intervals) {
  for (int i = 0; i < intervals.size(); ++i) {
    std::cout << " [" << intervals[i][0] << "," << intervals[i][1] << "] ";
  }
}

class Solution {
 public:
  std::vector<std::vector<int>> merge(
      std::vector<std::vector<int>>& intervals) {
    std::sort(intervals.begin(), intervals.end(),
              [](auto a, auto b) { return a[0] < b[0]; });
    std::vector<std::vector<int>> result;
    for (auto interval : intervals) {
      if (result.empty()) {
        result.push_back(interval);
      } else if (result[result.size() - 1][1] < interval[0]) {
        result.push_back(interval);
      } else {
        result[result.size() - 1][1] =
            std::max(result[result.size() - 1][1], interval[1]);
      }
    }
    return result;
  }
};

void Test(const char* case_name, std::vector<std::vector<int>>& intervals,
          std::vector<std::vector<int>> expected_value) {
  Solution solution;
  auto result = solution.merge(intervals);
  if (std::equal(result.begin(), result.end(), expected_value.begin(),
                 expected_value.end())) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test01() {
  std::vector<std::vector<int>> intervals = {{1, 3}, {2, 6}, {8, 10}, {15, 18}};
  std::vector<std::vector<int>> expected_value = {{1, 6}, {8, 10}, {15, 18}};
  Test("Test01", intervals, expected_value);
}

void Test02() {
  std::vector<std::vector<int>> intervals = {{1, 4}, {4, 5}};
  std::vector<std::vector<int>> expected_value = {{1, 5}};
  Test("Test02", intervals, expected_value);
}

int main() {
  Test01();
  Test02();
  return 0;
}
