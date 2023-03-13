#include <iostream>
#include <string>

// 接口类
class Subject {
 public:
  virtual void request() = 0;
};

// 真实主题类
class RealSubject : public Subject {
 public:
  void request() override { std::cout << "real subject request" << std::endl; }
};

// 代理类
class Proxy : public Subject {
 public:
  Proxy() { m_real_subject = new RealSubject(); }
  ~Proxy() { delete m_real_subject; }

  void request() override {
    std::cout << "proxy request" << std::endl;
    m_real_subject->request();
  }

 private:
  RealSubject* m_real_subject;
};

int main() {
  Proxy proxy;
  proxy.request();
  return 0;
}
