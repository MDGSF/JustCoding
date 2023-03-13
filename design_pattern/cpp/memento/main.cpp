#include <iostream>
#include <string>
#include <vector>

// 备忘录类
class Memento {
 public:
  Memento(const std::string& state) : m_state(state) {}

  std::string get_state() const { return m_state; }

 private:
  std::string m_state;
};

class Originator {
 public:
  void set_state(const std::string& state) { m_state = state; }

  std::string get_state() const { return m_state; }

  Memento save_state_to_memento() const { return Memento(m_state); }

  void restore_state_from_memento(const Memento& memento) {
    m_state = memento.get_state();
  }

 private:
  std::string m_state;
};

class CareTaker {
 public:
  void add_memento(const Memento& memento) { m_mementos.push_back(memento); }
  Memento get_memento(int index) const { return m_mementos[index]; }

 private:
  std::vector<Memento> m_mementos;
};

int main() {
  Originator originator;
  CareTaker caretaker;

  originator.set_state("state #1");
  originator.set_state("state #2");
  caretaker.add_memento(originator.save_state_to_memento());

  originator.set_state("state #3");
  caretaker.add_memento(originator.save_state_to_memento());

  originator.set_state("state #4");
  std::cout << "current state: " << originator.get_state() << std::endl;

  originator.restore_state_from_memento(caretaker.get_memento(1));
  std::cout << "current state: " << originator.get_state() << std::endl;

  originator.restore_state_from_memento(caretaker.get_memento(0));
  std::cout << "current state: " << originator.get_state() << std::endl;

  return 0;
}
