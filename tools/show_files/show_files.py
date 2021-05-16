#!/usr/bin/env python3
# -*- coding: UTF-8 -*-

import os
import subprocess

g_dir_path = ""
g_file_suffix = ".png"


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
  for file in sorted(files):
    if file[0] == '.':  # 隐藏文件
      continue
    file_path = os.path.join(directory, file)
    if os.path.isdir(file_path):
      continue
    if not file.endswith(suffix):
      continue
    print("\\newpage")
    print("\includegraphics[width=\\textwidth]{Images/" + "{}".format(file) + "}")


def main():
  process_dir(g_dir_path, g_file_suffix)


if __name__ == "__main__":
  main()
