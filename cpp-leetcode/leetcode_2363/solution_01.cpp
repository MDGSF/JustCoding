#include "cpplang.hpp"

class Solution {
 public:
  std::vector<std::vector<int>> mergeSimilarItems(
      std::vector<std::vector<int>>& items1,
      std::vector<std::vector<int>>& items2) {
    std::map<int, int> m;
    auto calc = [&m](std::vector<std::vector<int>>& items) {
      for (int i = 0; i < items.size(); ++i) {
        int value = items[i][0];
        int weight = items[i][1];
        if (m.find(value) == m.end()) {
          m[value] = weight;
        } else {
          m[value] += weight;
        }
      }
    };
    calc(items1);
    calc(items2);

    std::vector<std::vector<int>> result;
    for (auto it = m.begin(); it != m.end(); ++it) {
      result.push_back(std::vector<int>{it->first, it->second});
    }
    return result;
  }
};

void Test(const char* case_name, std::vector<std::vector<int>>& items1,
          std::vector<std::vector<int>>& items2,
          std::vector<std::vector<int>>& expected_value) {
  Solution solution;
  auto result = solution.mergeSimilarItems(items1, items2);
  if (std::equal(result.begin(), result.end(), expected_value.begin(),
                 expected_value.end())) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<std::vector<int>> items1{{1, 1}, {4, 5}, {3, 8}};
  std::vector<std::vector<int>> items2{{3, 1}, {1, 5}};
  std::vector<std::vector<int>> expected_value{{1, 6}, {3, 9}, {4, 5}};
  Test("Test1", items1, items2, expected_value);
}

void Test2() {
  std::vector<std::vector<int>> items1{{1, 1}, {3, 2}, {2, 3}};
  std::vector<std::vector<int>> items2{{2, 1}, {3, 2}, {1, 3}};
  std::vector<std::vector<int>> expected_value{{1, 4}, {2, 4}, {3, 4}};
  Test("Test2", items1, items2, expected_value);
}

void Test3() {
  std::vector<std::vector<int>> items1{{1, 3}, {2, 2}};
  std::vector<std::vector<int>> items2{{7, 1}, {2, 2}, {1, 4}};
  std::vector<std::vector<int>> expected_value{{1, 7}, {2, 4}, {7, 1}};
  Test("Test3", items1, items2, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  return 0;
}
