#include <iostream>
#include <string>

class State {
 public:
  virtual std::string name() = 0;
};

class StateA : public State {
 public:
  std::string name() { return "StateA"; }
};

class StateB : public State {
 public:
  std::string name() { return "StateB"; }
};

class Context {
 public:
  Context() { m_state = new StateA(); }
  ~Context() { delete m_state; }

  void set_state(State* state) {
    delete m_state;
    m_state = state;
  }

  void current_state() { std::cout << m_state->name() << std::endl; }

 private:
  State* m_state = nullptr;
};

int main() {
  Context context;
  context.current_state();
  context.set_state(new StateB());
  context.current_state();
  return 0;
}
