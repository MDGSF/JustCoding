#include <iostream>

std::string hex_to_deci(const std::string& hex) {
  int decimal = 0;
  for (auto c : hex) {
    if (c >= '0' && c <= '9') {
      decimal = decimal * 16 + c - '0';
    } else if (c >= 'a' && c <= 'z') {
      decimal = decimal * 16 + c - 'a' + 10;
    } else if (c >= 'A' && c <= 'Z') {
      decimal = decimal * 16 + c - 'A' + 10;
    }
  }
  return std::to_string(decimal);
}

int main() {
  std::string str;
  while (std::cin >> str) {
    std::string hex = str.substr(2, str.size() - 2);
    std::string deci = hex_to_deci(hex);
    std::cout << deci << std::endl;
  }
  return 0;
}
