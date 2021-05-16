#!/usr/bin/env python3
# -*- coding: UTF-8 -*-

import os
import subprocess

g_file_path = ""
g_suffix = ".png"


def remove_prefix(text, prefix):
  if text.startswith(prefix):
    return text[len(prefix):]
  return text


def remove_suffix(text, suffix):
  if text.endswith(suffix):
    return text[:-len(suffix)]
  return text


def process_dir(directory, suffix):
  files = os.listdir(directory)
  png_files = []
  for file in sorted(files):
    if file[0] == '.':  # 隐藏文件
      continue
    file_path = os.path.join(directory, file)
    if os.path.isdir(file_path):
      continue
    if not file.endswith(suffix):
      continue
    png_files.append(file)
  #move_back(directory, suffix, png_files, 3)
  fill_hols(directory, suffix, png_files)


def fill_hols(directory, suffix, png_files):
  png_files.sort()
  for i, file_name in enumerate(png_files):
    src_file_name = os.path.join(directory, file_name)
    dst_file_name = os.path.join(directory, '{:0>3d}'.format(i) + suffix)
    print(i, file_name, src_file_name, dst_file_name)
    os.rename(src_file_name, dst_file_name)


def move_back(directory, suffix, png_files, steps):
  png_files.sort(key = lambda x: int(remove_suffix(x, suffix)))
  #png_files.sort()
  for i in range(len(png_files) - 1, -1, -1):
    file_name = png_files[i]
    src_file_name = os.path.join(directory, file_name)
    dst_file_name = os.path.join(directory, '{:0>3d}'.format(i + steps) + suffix)
    print(i, file_name, src_file_name, dst_file_name)
    os.rename(src_file_name, dst_file_name)


def main():
  process_dir(g_file_path, g_suffix)


if __name__ == "__main__":
  main()
