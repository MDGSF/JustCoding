#include <iostream>
#include <map>
#include <vector>

class Solution {
 public:
  int equalPairs(std::vector<std::vector<int>>& grid) {
    std::map<std::vector<int>, int> row_map;  // <row, count>
    for (auto row : grid) {
      row_map[row] += 1;
    }

    int count = 0;
    int rows = grid.size();
    int cols = grid[0].size();
    for (int col = 0; col < cols; ++col) {
      std::vector<int> cur_col;
      for (int row = 0; row < rows; ++row) {
        cur_col.emplace_back(grid[row][col]);
      }

      if (row_map.find(cur_col) != row_map.end()) {
        count += row_map[cur_col];
      }
    }

    return count;
  }
};

void Test(const char* case_name, std::vector<std::vector<int>>& grid,
          int expected_value) {
  Solution solution;
  int result = solution.equalPairs(grid);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<std::vector<int>> grid = {{3, 2, 1}, {1, 7, 6}, {2, 7, 7}};
  int expected_value = 1;
  Test("Test1", grid, expected_value);
}

void Test2() {
  std::vector<std::vector<int>> grid = {
      {3, 1, 2, 2}, {1, 4, 4, 5}, {2, 4, 2, 2}, {2, 4, 2, 2}};
  int expected_value = 3;
  Test("Test2", grid, expected_value);
}

int main() {
  Test1();
  Test2();
  return 0;
}
