# HJ

## TODO

- HJ16
- HJ18

## cpp

### char

```cpp
char cc;
char c = std::tolower(cc);
char c = std::toupper(cc);

bool isalpha(char c) 是否是大小写字符
bool isalnum(char c) 是否是大小写字符，数字
bool islower(char c) 是否是小写字符
bool isupper(char c) 是否是大写字符
bool isdigit(char c) 是否是数字
```

### string

```cpp
// 读取stdin输入
std::cin >> str;
getline(std::cin, str);

// 构造 n 个 c 字符
std::string(int n, char c);

// 整数转字符串
std::to_string()

// 字符串反转
reverse(str.begin(), str.end());

// 查找
// 找到返回下标
// 找不到返回 std::string::npos
int index = find("needle")
rfind()

// 截取子字符串
std::string sub = str.substr(start_index, length);
std::string sub = str.substr(2, str.size() - 2); // 前两个字符不要

// 向字符串末尾插入字符
str.push_back(char c);

// 遍历字符串
for (char c : str) {}
```

### map

```cpp
// map 的 value 如果是 int，默认值是 0
std::map<std::string, int> m;

// map插入
m["hello"] = 123;

// 遍历map
for (auto it = m.begin(); it != m.end(); ++it) {
  std::cout << it->first << " " << it->second << std::endl;
}

// 遍历map
for (auto& it : m) {
  std::cout << it.first << " " << it.second << std::endl;
}
```

### set

```cpp
// string 存到 set 中
std::string str;
std::set<char> s(str.begin(), str.end());

// set插入
s.insert(num);
s.insert(str);

// 没有找到
if (s.find(num) == s.end()) {}
```

### vector

```cpp
// 从set初始化vector
std::vector<int> nums(s.begin(), s.eng());

// vector排序，从小到大
std::sort(nums.begin(), nums.end());

// vector排序，从大到小
std::vector<int> nums = {1,2,3,4,5};
std::sort(nums.begin(), nums.end(), [](int a, int b) { return a > b; });

// 从数组末尾插入
vector.push_back();
```
