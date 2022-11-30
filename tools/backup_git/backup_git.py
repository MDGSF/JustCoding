#!/usr/bin/env python3
# -*- coding: UTF-8 -*-

import os
import sys
import time

gits_filename = "gits.txt"
backup_dir = ""
gits = []

def process_one_git(one_git):
  print(one_git)
  os.system(f'git clone {one_git}')

def get_date():
  now = time.localtime()
  now_time = time.strftime("%Y%m%d_%H%M%S", now)
  return now_time

def check_dir():
  global backup_dir
  if backup_dir == "":
    backup_dir = os.path.join(os.getcwd(), f'backup_{get_date()}')
  print(f'backup_dir: {backup_dir}')
  if not os.path.exists(backup_dir):
    os.makedirs(backup_dir)
  os.chdir(backup_dir)
  print(f'working directory: {os.getcwd()}')

def read_list_file():
  global gits
  with open(gits_filename) as fp:
    lines = fp.readlines()
  for line in lines:
    gits.append(line.rstrip())
  print(gits)

def main():
  read_list_file()
  check_dir()
  for one_git in gits:
    process_one_git(one_git)

if __name__ == "__main__":
  main()
