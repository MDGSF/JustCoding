#include <iostream>
#include <cmath>

int main() {
  float f;
  std::cin >> f;
  std::cout << static_cast<int>(std::round(f)) << std::endl;
  return 0;
}
