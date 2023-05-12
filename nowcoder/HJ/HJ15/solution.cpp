#include <iostream>

int main() {
  int n;
  std::cin >> n;

  int count = 0;
  while (n > 0) {
    n = n & (n - 1);
    count += 1;
  }

  std::cout << count << std::endl;

  return 0;
}
