#include <iostream>
#include <vector>

class Solution {
 public:
  std::vector<int> spiralOrder(std::vector<std::vector<int>>& matrix) {
    int rows = matrix.size();
    int cols = matrix[0].size();
    int n = rows * cols;
    int left = 0;
    int right = cols - 1;
    int up = 0;
    int bottom = rows - 1;
    std::vector<int> result;
    while (true) {
      for (int col = left; col <= right; ++col) {
        result.push_back(matrix[up][col]);
      }
      up += 1;
      if (result.size() >= n) {
        break;
      }

      for (int row = up; row <= bottom; ++row) {
        result.push_back(matrix[row][right]);
      }
      right -= 1;
      if (result.size() >= n) {
        break;
      }

      for (int col = right; col >= left; --col) {
        result.push_back(matrix[bottom][col]);
      }
      bottom -= 1;
      if (result.size() >= n) {
        break;
      }

      for (int row = bottom; row >= up; --row) {
        result.push_back(matrix[row][left]);
      }
      left += 1;
      if (result.size() >= n) {
        break;
      }
    }
    return result;
  }
};

void Test(const char* case_name, std::vector<std::vector<int>>& matrix,
          std::vector<int>& expected_value) {
  Solution solution;
  auto result = solution.spiralOrder(matrix);
  if (std::equal(result.begin(), result.end(), expected_value.begin(),
                 expected_value.end())) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<std::vector<int>> matrix = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
  std::vector<int> expected_value = {1, 2, 3, 6, 9, 8, 7, 4, 5};
  Test("Test1", matrix, expected_value);
}

void Test2() {
  std::vector<std::vector<int>> matrix = {
      {1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}};
  std::vector<int> expected_value = {1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7};
  Test("Test2", matrix, expected_value);
}

int main() {
  Test1();
  Test2();
  return 0;
}
