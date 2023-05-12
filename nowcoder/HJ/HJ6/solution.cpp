#include <iostream>

// 会超时
void prime_factors_01(int n) {
  for (int i = 2; i <= n; ++i) {
    while (n % i == 0) {
      std::cout << i << " ";
      n /= i;
    }
  }

  std::cout << std::endl;
}

void prime_factors(int n) {
  for (int i = 2; i * i <= n; ++i) {
    while (n % i == 0) {
      std::cout << i << " ";
      n /= i;
    }
  }
  if (n > 1) {  // 如果n是大于1的质数，直接输出n
    std::cout << n << " ";
  }
  std::cout << std::endl;
}

int main() {
  int n = 0;
  std::cin >> n;
  prime_factors(n);
  return 0;
}
