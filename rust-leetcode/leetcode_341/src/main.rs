use std::collections::VecDeque;

// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
struct NestedIterator {
  nested_list: VecDeque<NestedInteger>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
  fn new(nested_list: Vec<NestedInteger>) -> Self {
    NestedIterator {
      nested_list: nested_list.into_iter().collect(),
    }
  }

  fn next(&mut self) -> i32 {
    self.expand_nested();

    let first = self.nested_list.pop_front();
    let nested = match first {
      Some(nested) => nested,
      None => return 0
    };

    if let NestedInteger::Int(integer) = nested {
      return integer;
    }

    0
  }

  fn expand_nested(&mut self) {
    let first = self.nested_list.pop_front();
    let nested = match first {
      Some(nested) => nested,
      None => return,
    };
    match nested {
      NestedInteger::Int(_) => self.nested_list.push_front(nested),
      NestedInteger::List(nested_list) => {
        for num in nested_list.into_iter().rev() {
          self.nested_list.push_front(num);
        }
        self.expand_nested();
      },
    };
  }

  fn has_next(&mut self) -> bool {
    self.expand_nested();
    if self.nested_list.is_empty() {
      return false;
    }
    true
  }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nested_list);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>),
}

fn main() {
  let n1 = NestedInteger::Int(1);
  let n2 = NestedInteger::Int(1);
  let n3 = vec![n1, n2];
  let n4 = NestedInteger::List(n3);

  let n5 = NestedInteger::Int(2);

  let n6 = NestedInteger::Int(1);
  let n7 = NestedInteger::Int(1);
  let n8 = vec![n6, n7];
  let n9 = NestedInteger::List(n8);

  let n10 = vec![n4, n5, n9];
  let mut obj = NestedIterator::new(n10);
  while obj.has_next() {
    println!("{:?}", obj.next());
  }
}
