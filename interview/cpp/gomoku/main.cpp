#include <cmath>
#include <iostream>
#include <string>
#include <vector>

/*
编写一个函数，计算一个只有黑色棋子的棋盘上至少要再放几颗黑子才能达到五子连珠：
int gomoku(const std::vector<std::vector<bool>> &board);
说明：
1. 棋盘上除了空白就只有黑子
2. board 表示一个棋盘，board[i][j] == true
表示第i行、第j列有一颗黑子，否则表示为空白
3. 保证 行数>=5，列数>=5
*/
int gomoku(const std::vector<std::vector<bool>>& board) {
  int rows = board.size();
  int cols = board[0].size();
  int min_added = 5;
  for (int row = 0; row < rows; ++row) {
    for (int col = 0; col < cols; ++col) {
      {
        // 以 board[row][col] 为起点，向右数 5 个
        int count = 0;
        for (int i = 0; i < 5; ++i) {
          int cur_row = row;
          int cur_col = col + i;
          if (cur_row >= rows || cur_col >= cols) {
            break;
          }
          if (board[cur_row][cur_col]) {
            count += 1;
          }
        }
        if (count == 5) {
          return 0;
        } else {
          min_added = std::min(min_added, 5 - count);
        }
      }

      {
        // 以 board[row][col] 为起点，向下数 5 个
        int count = 0;
        for (int i = 0; i < 5; ++i) {
          int cur_row = row + i;
          int cur_col = col;
          if (cur_row >= rows || cur_col >= cols) {
            break;
          }
          if (board[cur_row][cur_col]) {
            count += 1;
          }
        }
        if (count == 5) {
          return 0;
        } else {
          min_added = std::min(min_added, 5 - count);
        }
      }

      {
        // 以 board[row][col] 为起点，向右下数 5 个
        int count = 0;
        for (int i = 0; i < 5; ++i) {
          int cur_row = row + i;
          int cur_col = col + i;
          if (cur_row >= rows || cur_col >= cols) {
            break;
          }
          if (board[cur_row][cur_col]) {
            count += 1;
          }
        }
        if (count == 5) {
          return 0;
        } else {
          min_added = std::min(min_added, 5 - count);
        }
      }

      {
        // 以 board[row][col] 为起点，向左下数 5 个
        int count = 0;
        for (int i = 0; i < 5; ++i) {
          int cur_row = row + i;
          int cur_col = col - i;
          if (cur_row >= rows || cur_col < 0) {
            break;
          }
          if (board[cur_row][cur_col]) {
            count += 1;
          }
        }
        if (count == 5) {
          return 0;
        } else {
          min_added = std::min(min_added, 5 - count);
        }
      }
    }
  }
  return min_added;
}

void test(const std::string& name, const std::vector<std::vector<bool>>& board,
          int expected_result) {
  int result = gomoku(board);
  if (result == expected_result) {
    std::cout << "Pass " << name << std::endl;
  } else {
    std::cout << "Fail " << name << ", expected_result:" << expected_result
              << ", result:" << result << std::endl;
  }
}

void test1() {
  // clang-format off
  std::vector<std::vector<bool>> board{
      {false, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
  };
  // clang-format on
  test("test1", board, 5);
}

void test2() {
  // clang-format off
  std::vector<std::vector<bool>> board{
      {true, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
  };
  // clang-format on
  test("test2", board, 4);
}

void test3() {
  // clang-format off
  std::vector<std::vector<bool>> board{
      {true, true, false, true, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
  };
  // clang-format on
  test("test3", board, 2);
}

void test4() {
  // clang-format off
  std::vector<std::vector<bool>> board{
      {true, false, false, true, false},
      {false, true, false, false, false},
      {false, false, false, false, false},
      {false, false, false, true, false},
      {false, false, false, false, true},
  };
  // clang-format on
  test("test4", board, 1);
}

void test5() {
  // clang-format off
  std::vector<std::vector<bool>> board{
      {false, false, false, true, false},
      {false, false, false, false, false},
      {false, false, false, false, false},
      {false, false, false, true, false},
      {false, false, false, false, true},
  };
  // clang-format on
  test("test5", board, 3);
}

int main() {
  test1();
  test2();
  test3();
  test4();
  test5();
  return 0;
}
