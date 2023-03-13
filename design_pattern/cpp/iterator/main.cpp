#include <iostream>
#include <vector>

class Iterator {
 public:
  virtual int get_next() = 0;
  virtual bool has_more() = 0;
};

class IntArrayIterator : public Iterator {
 public:
  IntArrayIterator(std::vector<int>& array) : m_array(array) {}

  int get_next() override { return m_array[m_index++]; }
  bool has_more() override { return m_index < m_array.size(); }

 private:
  std::vector<int>& m_array;
  int m_index = 0;
};

class Container {
 public:
  virtual Iterator* get_iterator() = 0;
};

class IntArray : public Container {
 public:
  IntArray(std::vector<int> array) : m_array(array) {}

  Iterator* get_iterator() override { return new IntArrayIterator(m_array); }

 private:
  std::vector<int> m_array;
};

int main() {
  std::vector<int> array = {1, 2, 3, 4, 5};
  Container* container = new IntArray(array);
  Iterator* iterator = container->get_iterator();
  while (iterator->has_more()) {
    std::cout << iterator->get_next() << " ";
  }
  std::cout << std::endl;

  delete container;
  delete iterator;
  return 0;
}
