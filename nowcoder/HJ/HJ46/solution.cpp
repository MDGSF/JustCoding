#include <iostream>

int main() {
  std::string str;
  getline(std::cin, str);

  int k;
  std::cin >> k;

  std::string sub = str.substr(0, k);

  std::cout << sub << std::endl;

  return 0;
}
