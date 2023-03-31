#include <iostream>
#include <string>
#include <cmath>

std::string hex_to_decimal(const std::string& hex) {
  int decimal = 0;
  int n = hex.length();
  for (int i = 0; i < n; ++i) {
    int digit = 0;
    if (hex[i] >= '0' && hex[i] <= '9') {
      digit = hex[i] - '0';
    } else if (hex[i] >= 'A' && hex[i] <= 'F') {
      digit = hex[i] - 'A' + 10;
    } else if (hex[i] >= 'a' && hex[i] <= 'f') {
      digit = hex[i] - 'a' + 10;
    } else {
      throw std::invalid_argument("invalid parameter");
    }
    decimal += digit * pow(16, n - i - 1);
  }
  return std::to_string(decimal);
}

void test(const std::string& name, const std::string& hex,
          const std::string& expected_decimal) {
  std::string decimal = hex_to_decimal(hex);
  if (decimal == expected_decimal) {
    std::cout << "Pass " << name << std::endl;
  } else {
    std::cout << "Fail " << name << ", expected_decimal:" << expected_decimal
              << ", decimal:" << decimal << std::endl;
  }
}

void test1() {
  test("test1", "1", "1");
}

void test2() {
  test("test2", "A", "10");
}

void test3() {
  test("test3", "A1", "161");
}

void test4() {
  test("test4", "10", "16");
}

void test5() {
  test("test5", "0F", "15");
}

int main() {
  test1();
  test2();
  test3();
  test4();
  test5();
  return 0;
}
