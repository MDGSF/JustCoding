#include <iostream>
#include <string>

int main() {
  std::string str;
  getline(std::cin, str);
  int alpha = 0;
  int blank = 0;
  int number = 0;
  int other = 0;
  for (char c : str) {
    if (std::isalpha(c)) {
      alpha += 1;
    } else if(c == ' ') {
      blank += 1;
    } else if (std::isdigit(c)) {
      number += 1;
    } else {
      other += 1;
    }
  }
  std::cout << alpha << std::endl;
  std::cout << blank << std::endl;
  std::cout << number << std::endl;
  std::cout << other << std::endl;
  return 0;
}
