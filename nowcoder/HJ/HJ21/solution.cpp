#include <iostream>
#include <string>

int main() {
  std::string str;
  std::cin >> str;

  for (auto c : str) {
    if (c >= 'a' && c <= 'z') {
      if (c >= 'a' && c <= 'c') {
        printf("2");
      } else if (c >= 'd' && c <= 'f') {
        printf("3");
      } else if (c >= 'g' && c <= 'i') {
        printf("4");
      } else if (c >= 'j' && c <= 'l') {
        printf("5");
      } else if (c >= 'm' && c <= 'o') {
        printf("6");
      } else if (c >= 'p' && c <= 's') {
        printf("7");
      } else if (c >= 't' && c <= 'v') {
        printf("8");
      } else if (c >= 'w' && c <= 'z') {
        printf("9");
      }
    } else if (c >= 'A' && c <= 'Z') {
      char t = (c - 'A' + 1) % 26 + 'a';
      printf("%c", t);
    } else {
      printf("%c", c);
    }
  }

  return 0;
}
