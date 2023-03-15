#include <iostream>
#include <cstring>

void* my_memmove(void* dst, const void* src, size_t size) {
  if (dst == nullptr || src == nullptr) {
    return nullptr;
  }

  if ((src < dst) && (char*)src + size > (char*)dst) {
    // 从后向前拷贝
    char* psrc = (char*)src + size - 1;
    char* pdst = (char*)dst + size - 1;
    while (size--) {
      *pdst-- = *psrc--;
    }
  } else {
    // 从前向后拷贝
    char* psrc = (char*)src;
    char* pdst = (char*)dst;
    while (size--) {
      *pdst++ = *psrc++;
    }
  }

  return dst;
}

int main() {
  const char* a = "hello";
  char b[1024] = {0};
  my_memmove(b, a, strlen(a) + 1);
  my_memmove(b+3, b, strlen(a) + 1);
  printf("%s\n", b);
  return 0;
}
