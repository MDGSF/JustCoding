#include <iostream>
#include <vector>

template <class TParam>
class SlotBase {
 public:
  virtual ~SlotBase() = default;
  virtual void slot_function(TParam) = 0;
};

template <class TReceiver, class TParam>
class Slot : public SlotBase<TParam> {
 public:
  Slot(TReceiver* receiver, void (TReceiver::*func)(TParam)) {
    m_receiver = receiver;
    m_func = func;
  }

  void slot_function(TParam param) override { (m_receiver->*m_func)(param); }

 private:
  // 定义一个接收者的指针
  TReceiver* m_receiver = nullptr;
  // 定义一个接收者类中的成员函数指针
  void (TReceiver::*m_func)(TParam param) = nullptr;
};

template <class TParam>
class Signal {
 public:
  template <class TReceiver>
  void add_slot(TReceiver* receiver, void (TReceiver::*func)(TParam)) {
    m_signals.push_back(new Slot<TReceiver, TParam>(receiver, func));
  }

  void operator()(TParam param) {
    for (SlotBase<TParam>* p : m_signals) {
      p->slot_function(param);
    }
  }

 private:
  std::vector<SlotBase<TParam>*> m_signals;
};

class Receiver1 {
 public:
  void func1(int param) {
    std::cout << "Receiver1, param = " << param << std::endl;
  }
};

class Receiver2 {
 public:
  void func2(int param) {
    std::cout << "Receiver2, param = " << param << std::endl;
  }
};

class SendObj {
 public:
  Signal<int> value_changed;

  void test_signal(int value) { value_changed(value); }
};

#define connect(sender, signal, receiver, method) \
  (sender)->signal.add_slot(receiver, method)

int main() {
  Receiver1* r1 = new Receiver1;
  Receiver2* r2 = new Receiver2;
  SendObj* sd = new SendObj;
  connect(sd, value_changed, r1, &Receiver1::func1);
  connect(sd, value_changed, r2, &Receiver2::func2);
  sd->test_signal(10000);
  return 0;
}
