#include <iostream>

// 最大公约数
int gcd(int a, int b) {
  if (b == 0) {
    return a;
  }
  return gcd(b, a % b);
}

// 最小公倍数
int lcm(int a, int b) {
  return a * b / gcd(a, b);
}

int main() {
  int a;
  int b;
  std::cin >> a >> b;
  std::cout << lcm(a, b) << std::endl;
  return 0;
}
