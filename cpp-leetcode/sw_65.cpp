#include <stdio.h>

int add(int a, int b) { return ((unsigned int)(a & b) << 1) + (a ^ b); }

int add1(int a, int b) {
  while (b != 0) {
    int sum = (a ^ b);
    int carry = (unsigned int)(a & b) << 1;
    a = sum;
    b = carry;
  }
  return a;
}

int main() {
  int result = add(-1, 2);
  printf("result = %d\n", result);
  return 0;
}
