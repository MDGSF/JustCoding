#!/usr/bin/env python

import sys

for str1 in sys.stdin:
  str2 = str1.split()[-1]
  str2_len = len(str2)
  print(str2_len)
