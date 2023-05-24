#include <algorithm>
#include <iostream>
#include <string>

int main() {
  std::string str = "hello World";
  std::reverse(str.begin(), str.end());
  std::transform(str.begin(), str.end(), str.begin(), ::tolower);
  std::cout << str << std::endl;
  return 0;
}
