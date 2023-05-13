/*
n=1 -1
n=2 -1
n=3 2
n=4 3
n=5 2
n=6 4
n=7 2
n=8 3
n=9 2
n=10 4
*/

#include <iostream>

int main() {
  int n;
  std::cin >> n;
  if (n == 1 || n == 2) {
    std::cout << -1 << std::endl;
  } else if (n % 4 == 3 || n % 4 == 1) {
    std::cout << 2 << std::endl;
  } else if (n % 4 == 0) {
    std::cout << 3 << std::endl;
  } else if (n % 4 == 2) {
    std::cout << 4 << std::endl;
  }
  return 0;
}
