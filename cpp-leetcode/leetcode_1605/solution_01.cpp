#include "cpplang.hpp"

class Solution {
 public:
  std::vector<std::vector<int>> restoreMatrix(std::vector<int>& rowSum,
                                              std::vector<int>& colSum) {
    int rows = rowSum.size();
    int cols = colSum.size();
    std::vector<std::vector<int>> matrix(rows, std::vector<int>(cols));
    return matrix;
  }
};

bool is_valid(std::vector<int>& rowSum, std::vector<int>& colSum,
              const std::vector<std::vector<int>>& matrix) {
  int rows = matrix.size();
  int cols = matrix[0].size();

  for (int row = 0; row < rows; ++row) {
    int sum = 0;
    for (int col = 0; col < cols; ++col) {
      sum += matrix[row][col];
    }
    if (sum != rowSum[row]) {
      return false;
    }
  }

  for (int col = 0; col < cols; ++col) {
    int sum = 0;
    for (int row = 0; row < rows; ++row) {
      sum += matrix[row][col];
    }
    if (sum != colSum[col]) {
      return false;
    }
  }

  return true;
}

void Test(const char* case_name, std::vector<int>& rowSum,
          std::vector<int>& colSum) {
  Solution solution;
  auto result = solution.restoreMatrix(rowSum, colSum);
  if (is_valid(rowSum, colSum, result)) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

/*
  4  7
3 0  3
8 4  4
*/
void Test1() {
  std::vector<int> rowSum{3, 8};
  std::vector<int> colSum{4, 7};
  Test("Test1", rowSum, colSum);
}

int main() {
  Test1();
  return 0;
}
