/*
1 1
2 3 = 2*2-1
3 5 = 2*3-1
4 7 = 2*4-1
*/

#include <iostream>
#include <vector>

int main() {
  int rows;
  std::cin >> rows;
  int size = 2 * rows - 1;
  std::vector<int> one(size, 0);
  std::vector<int> two(size, 0);
  std::vector<int>& pre = one;
  std::vector<int>& cur = two;
  pre[0] = 1;
  for (int row = 2; row <= rows; ++row) {
    int row_size = 2 * row - 1;
    for (int i = 0; i < row_size; ++i) {
      if (i == 0 || i == row_size - 1) {
        cur[i] = 1;
      } else if (i == 1) {
        cur[i] = pre[0] + pre[1];
      } else {
        cur[i] = pre[i] + pre[i - 1] + pre[i - 2];
      }
    }
    std::swap(pre, cur);
  }

  bool found = false;
  for (int i = 0; i < size; ++i) {
    if (pre[i] % 2 == 0) {
      std::cout << i + 1 << std::endl;
      found = true;
      break;
    }
  }
  if (!found) {
    std::cout << -1 << std::endl;
  }

  return 0;
}
