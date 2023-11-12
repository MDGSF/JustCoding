#include "cpplang.hpp"

class Solution {
public:
  bool validIPv4(const std::string &queryIP) {
    int len = queryIP.size();
    if (len < 7 || len > 15) {
      return false;
    }

    int start = 0;
    int end = 0;
    int dotCount = 0;
    int numCount = 0;
    while (start < len) {
      end = queryIP.find(".", start);
      if (end == std::string::npos) {
        if (dotCount != 3) {
          return false;
        }
        end = len;
      } else {
        dotCount += 1;
      }

      int diff = end - start;
      if (diff < 1 || diff > 3) {
        return false;
      }

      if (diff > 1 && queryIP[start] == '0') {
        return false;
      }

      int num = 0;
      for (int i = start; i < end; ++i) {
        if (!std::isdigit(queryIP[i])) {
          return false;
        }
        num = num * 10 + (queryIP[i] - '0');
      }
      if (num > 255) {
        return false;
      }

      numCount += 1;
      start = end + 1;
    }

    if (dotCount != 3 || numCount != 4) {
      return false;
    }

    return true;
  }

  bool validIPv6(const std::string &queryIP) {
    int len = queryIP.size();
    if (len < 15 || len > 39) {
      return false;
    }

    int start = 0;
    int end = 0;
    int dotCount = 0;
    int numCount = 0;
    while (start < len) {
      end = queryIP.find(":", start);
      if (end == std::string::npos) {
        if (dotCount != 7) {
          return false;
        }
        end = len;
      } else {
        dotCount += 1;
      }

      int diff = end - start;
      if (diff < 1 || diff > 4) {
        return false;
      }

      for (int i = start; i < end; ++i) {
        if (!(std::isdigit(queryIP[i]) ||
            (queryIP[i] >= 'a' && queryIP[i] <= 'f') ||
            (queryIP[i] >= 'A' && queryIP[i] <= 'F'))) {
          return false;
        }
      }

      numCount += 1;
      start = end + 1;
    }

    if (dotCount != 7 || numCount != 8) {
      return false;
    }

    return true;
  }

  std::string validIPAddress(std::string queryIP) {
    if (validIPv4(queryIP)) {
      return "IPv4";
    } else if (validIPv6(queryIP)) {
      return "IPv6";
    }
    return "Neither";
  }
};

void Test(const char *case_name, std::string queryIP,
          std::string expected_value) {
  Solution solution;
  auto result = solution.validIPAddress(queryIP);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::string queryIP = "172.16.254.1";
  std::string expected_value = "IPv4";
  Test("Test1", queryIP, expected_value);
}

void Test2() {
  std::string queryIP = "2001:0db8:85a3:0:0:8A2E:0370:7334";
  std::string expected_value = "IPv6";
  Test("Test2", queryIP, expected_value);
}

void Test3() {
  std::string queryIP = "256.256.256.256";
  std::string expected_value = "Neither";
  Test("Test3", queryIP, expected_value);
}

void Test4() {
  std::string queryIP = "127.0.0.1";
  std::string expected_value = "IPv4";
  Test("Test4", queryIP, expected_value);
}

void Test5() {
  std::string queryIP = "127.00.0.1";
  std::string expected_value = "Neither";
  Test("Test5", queryIP, expected_value);
}

void Test6() {
  std::string queryIP = "127.10.10.";
  std::string expected_value = "Neither";
  Test("Test6", queryIP, expected_value);
}

void Test7() {
  std::string queryIP = "2001:0db8:85a3::8A2E:037j:7334";
  std::string expected_value = "Neither";
  Test("Test7", queryIP, expected_value);
}

void Test8() {
  std::string queryIP = "02001:0db8:85a3:0000:0000:8a2e:0370:7334";
  std::string expected_value = "Neither";
  Test("Test8", queryIP, expected_value);
}

void Test9() {
  std::string queryIP = "2001:0db8:85a3:0:0:8A2E:0370:7334:";
  std::string expected_value = "Neither";
  Test("Test9", queryIP, expected_value);
}

void Test10() {
  std::string queryIP = "20EE:FGb8:85a3:0:0:8A2E:0370:7334";
  std::string expected_value = "Neither";
  Test("Test10", queryIP, expected_value);
}

void Test11() {
  std::string queryIP = "222.2f.22.1";
  std::string expected_value = "Neither";
  Test("Test11", queryIP, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  Test4();
  Test5();
  Test6();
  Test7();
  Test8();
  Test9();
  Test10();
  Test11();
  return 0;
}
