struct MyCircularDeque {
  array: Vec<i32>,
  head: usize,
  rear: usize,
  len: usize,
  capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
  /** Initialize your data structure here. Set the size of the deque to be k. */
  fn new(k: i32) -> Self {
    MyCircularDeque {
      array: vec![0; k as usize],
      head: 0,
      rear: 0,
      len: 0,
      capacity: k as usize,
    }
  }

  /** Adds an item at the front of Deque. Return true if the operation is successful. */
  fn insert_front(&mut self, value: i32) -> bool {
    if self.is_full() {
      return false;
    }
    self.array[self.head] = value;
    self.head = (self.head + 1) % self.capacity;
    self.len += 1;
    true
  }

  /** Adds an item at the rear of Deque. Return true if the operation is successful. */
  fn insert_last(&mut self, value: i32) -> bool {
    if self.is_full() {
      return false;
    }
    if self.rear == 0 {
      self.rear = self.capacity - 1;
    } else {
      self.rear -= 1;
    }
    self.array[self.rear] = value;
    self.len += 1;
    true
  }

  /** Deletes an item from the front of Deque. Return true if the operation is successful. */
  fn delete_front(&mut self) -> bool {
    if self.is_empty() {
      return false;
    }
    if self.head == 0 {
      self.head = self.capacity - 1;
    } else {
      self.head -= 1;
    }
    self.len -= 1;
    true
  }

  /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
  fn delete_last(&mut self) -> bool {
    if self.is_empty() {
      return false;
    }
    self.rear = (self.rear + 1) % self.capacity;
    self.len -= 1;
    true
  }

  /** Get the front item from the deque. */
  fn get_front(&self) -> i32 {
    if self.is_empty() {
      return -1;
    }
    let prev = if self.head == 0 {
      self.capacity - 1
    } else {
      self.head - 1
    };
    self.array[prev]
  }

  /** Get the last item from the deque. */
  fn get_rear(&self) -> i32 {
    if self.is_empty() {
      return -1;
    }
    self.array[self.rear]
  }

  /** Checks whether the circular deque is empty or not. */
  fn is_empty(&self) -> bool {
    self.len == 0
  }

  /** Checks whether the circular deque is full or not. */
  fn is_full(&self) -> bool {
    self.len == self.capacity
  }
}
