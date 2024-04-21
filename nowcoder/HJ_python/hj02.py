#!/usr/bin/env python

str1 = input()
str2 = input()
str1_lower = str1.lower()
str2_lower = str2.lower()
str2_times = str1_lower.count(str2_lower)
print(str2_times)
