#include <stdio.h>
#include <stdlib.h>
#include <iostream>

void skipBlank(const char** str);
bool scanUnsignedInteger(const char** str);
bool scanInteger(const char** str);

/*
数字的格式可以是 A[.[B]][e|EC] 或者 .B[e|EC]
A 是整数，可以有正负号
B 是一个无符号整数
C 是整数，可以有正负号
*/
bool isNumber(const char* str) {
  if (str == NULL) {
    return false;
  }

  skipBlank(&str);

  bool numeric = scanInteger(&str);

  if (*str == '.') {
    ++str;
    numeric = scanUnsignedInteger(&str) || numeric;
  }

  if (*str == 'e' || *str == 'E') {
    ++str;
    numeric = numeric && scanInteger(&str);
  }

  skipBlank(&str);

  return numeric && *str == '\0';
}

void skipBlank(const char** str) {
  while (**str == ' ') {
    ++(*str);
  }
}

bool scanUnsignedInteger(const char** str) {
  const char* before = *str;
  while (**str != '\0' && **str >= '0' && **str <= '9') {
    ++(*str);
  }
  return *str > before;
}

bool scanInteger(const char** str) {
  if (**str == '+' || **str == '-') {
    ++(*str);
  }
  return scanUnsignedInteger(str);
}

void Test(const char* testName, const char* str, bool expected) {
  if (testName != NULL) {
    printf("%s begins: ", testName);
  }
  if (isNumber(str) == expected) {
    printf("Passed.\n");
  } else {
    printf("FAILED.\n");
  }
}

int main() {
  Test("Test1", "100", true);
  Test("Test2", "123.45e+6", true);
  Test("Test3", "500", true);
  Test("Test4", "5e2", true);
  Test("Test5", "3.1416", true);
  Test("Test6", "600.", true);
  Test("Test7", "-.123", true);
  Test("Test8", "-1E-16", true);
  Test("Test9", "1.79769313486232E+308", true);

  printf("\n\n");

  Test("Test10", "12e", false);
  Test("Test11", "1a3.14", false);
  Test("Test12", "1+23", false);
  Test("Test13", "1.2.3", false);
  Test("Test14", "+-5", false);
  Test("Test15", "12e+5.4", false);
  Test("Test16", ".", false);
  Test("Test17", ".e1", false);
  Test("Test18", "e1", false);
  Test("Test19", "+.", false);
  Test("Test20", "", false);
  Test("Test21", NULL, false);

  printf("\n\n");

  Test("Test22", "1 ", true);

  return 0;
}
