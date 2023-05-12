#!/usr/bin/env bash

# -S 指定源码目录
# -B 指定编译目录
cmake \
  -S . \
  -B build

# 使用 make 进行编译
cmake --build build

# 运行
./build/solution
