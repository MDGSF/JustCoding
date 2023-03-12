#include <iostream>
#include <string>

template <typename T>
class Singleton {
 public:
  static T& GetInstance() {
    static T instance;
    return instance;
  }

 private:
  Singleton() = default;
  Singleton(const Singleton&) = delete;
  Singleton& operator=(const Singleton&) = delete;
};

class MyClass {
 public:
  void DoSomething() { std::cout << "MyClass DoSomething" << std::endl; }

 private:
  MyClass() = default;
  friend class Singleton<MyClass>;
};

int main() {
  {
    MyClass& instance = Singleton<MyClass>::GetInstance();
    instance.DoSomething();
  }

  {
    Singleton<MyClass>::GetInstance().DoSomething();
  }
  return 0;
}
