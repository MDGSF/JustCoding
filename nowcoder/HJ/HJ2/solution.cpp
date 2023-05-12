#include <algorithm>
#include <iostream>
#include <string>

int count_character_01(const std::string& str, char cc) {
  char c = std::tolower(cc);
  int count = 0;
  for (auto curc : str) {
    if (std::tolower(curc) == c) {
      count += 1;
    }
  }
  return count;
}

int count_character(const std::string& str, char cc) {
  char c = std::tolower(cc);
  return std::count_if(str.begin(), str.end(),
                       [c](char curc) { return std::tolower(curc) == c; });
}

int main() {
  std::string str;
  char cc;
  getline(std::cin, str);
  std::cin >> cc;

  int count = count_character(str, cc);

  std::cout << count << std::endl;

  return 0;
}
