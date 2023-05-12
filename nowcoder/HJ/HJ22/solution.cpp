#include <iostream>

int calc(int n) {
  int bottle_count = 0;
  while (n >= 3) {
    int new_bottle = n / 3;
    bottle_count += new_bottle;
    n = n % 3 + new_bottle;
  }
  if (n == 2) {
    bottle_count += 1;
  }
  return bottle_count;
}

int main() {
  int n;
  while (std::cin >> n && n > 0) {
    std::cout << calc(n) << std::endl;
  }
  return 0;
}
