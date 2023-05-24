#include <algorithm>
#include <iostream>
#include <numeric>

std::string test(const std::string& str) {
  return std::accumulate(str.rbegin(), str.rend(), std::string{},
                         [](std::string& str, char c) -> std::string& {
                           str.push_back(std::isupper(c) ? std::tolower(c) : c);
                           return str;
                         });
}

int main() {
  std::string str = "helloWorld";
  std::string result = test(str);
  std::cout << result << std::endl;
  return 0;
}
