#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

// 观察者
class Observer {
 public:
  virtual void update(int value) = 0;
};

// 被观察者
class Subject {
 public:
  void attach(Observer* obs) { m_observers.push_back(obs); }

  void detach(Observer* obs) {
    auto it = std::find(m_observers.begin(), m_observers.end(), obs);
    if (it != m_observers.end()) {
      m_observers.erase(it);
    }
  }

  void notify(int value) {
    for (auto obs : m_observers) {
      obs->update(value);
    }
  }

 private:
  std::vector<Observer*> m_observers;
};

class ConcreteObserver1 : public Observer {
 public:
  void update(int value) {
    std::cout << "ConcreteObserver1 value: " << value << std::endl;
  }
};

class ConcreteObserver2 : public Observer {
 public:
  void update(int value) {
    std::cout << "ConcreteObserver2 value: " << value << std::endl;
  }
};

int main() {
  Subject subject;

  ConcreteObserver1 observer1;
  ConcreteObserver2 observer2;

  subject.attach(&observer1);
  subject.attach(&observer2);

  subject.notify(42);

  subject.detach(&observer2);

  subject.notify(17);

  return 0;
}
