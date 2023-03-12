#include <iostream>
#include <map>

class Flyweight {
 public:
  virtual void operation() = 0;
};

class ConcreteFlyweight : public Flyweight {
 public:
  ConcreteFlyweight(int intrinsic_state) {
    m_intrinsic_state = intrinsic_state;
  }

  void operation() override {
    std::cout << "ConcreteFlyweight with intrinsic state: " << m_intrinsic_state
              << std::endl;
  }

 private:
  int m_intrinsic_state = 0;
};

class FlyweightFactory {
 public:
  Flyweight* get_flyweight(int intrinsic_state) {
    if (m_flyweights.find(intrinsic_state) == m_flyweights.end()) {
      m_flyweights[intrinsic_state] = new ConcreteFlyweight(intrinsic_state);
    }
    return m_flyweights[intrinsic_state];
  }

 private:
  std::map<int, Flyweight*> m_flyweights;
};

int main() {
  FlyweightFactory* factory = new FlyweightFactory();

  Flyweight* flyweight1 = factory->get_flyweight(1);
  flyweight1->operation();

  Flyweight* flyweight2 = factory->get_flyweight(2);
  flyweight2->operation();

  Flyweight* flyweight3 = factory->get_flyweight(3);
  flyweight3->operation();

  delete factory;
  delete flyweight1;
  delete flyweight2;
  delete flyweight3;

  return 0;
}
