#include <iostream>

int main() {
  float f;
  std::cin >> f;
  std::cout << static_cast<int>(f + 0.5) << std::endl;
  return 0;
}
