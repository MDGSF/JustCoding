#include <iostream>
#include <string>

void process_token(const std::string& token, int& x, int& y) {
  if (token.size() < 2) {
    return;
  }

  for (int i = 1; i < token.size(); ++i) {
    if (!std::isdigit(token[i])) {
      return;
    }
  }

  std::string num_str = token.substr(1, token.size() - 1);
  int num = std::stoi(num_str);

  char dir = token[0];
  if (dir == 'A') {
    x -= num;
  } else if (dir == 'D') {
    x += num;
  } else if (dir == 'W') {
    y += num;
  } else if (dir == 'S') {
    y -= num;
  } else {
  }
}

int main() {
  std::string str;
  std::cin >> str;

  int x = 0;
  int y = 0;

  int start = 0;
  for (int i = 0; i < str.size(); ++i) {
    if (str[i] == ';') {
      int token_len = i - start;
      if (token_len > 0) {
        std::string token = str.substr(start, token_len);
        process_token(token, x, y);
      }
      start = i + 1;
    }
  }
  int token_len = str.size() - start;
  if (token_len > 0) {
    std::string token = str.substr(start, str.size() - start);
    process_token(token, x, y);
  }

  std::cout << x << "," << y << std::endl;

  return 0;
}
