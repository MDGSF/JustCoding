#!/usr/bin/env python3

def is_ugly_number(num):
  if num <= 0:
    return False
  while num % 2 == 0:
    num //= 2
  while num % 3 == 0:
    num //= 3
  while num % 5 == 0:
    num //= 5
  return num == 1

def main():
  for i in range(10):
    print(f'{i}: {is_ugly_number(i)}')

if __name__ == "__main__":
  main()
