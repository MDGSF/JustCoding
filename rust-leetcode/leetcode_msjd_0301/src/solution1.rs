struct TripleInOne {
    stack_size: usize,
    stack: Vec<i32>,
    stack_idxs: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TripleInOne {
    fn new(stackSize: i32) -> Self {
        TripleInOne {
            stack_size: stackSize as usize,
            stack: vec![0; (stackSize * 3) as usize],
            stack_idxs: vec![0, stackSize as usize, 2 * stackSize as usize],
        }
    }

    fn push(&mut self, stack_num: i32, value: i32) {
        let stack_num = stack_num as usize;
        let start_idx = stack_num * self.stack_size;
        let end_idx = start_idx + self.stack_size;
        let stack_idx = self.stack_idxs[stack_num];
        if stack_idx >= end_idx {
            return;
        }
        self.stack[stack_idx] = value;
        self.stack_idxs[stack_num] += 1;
    }

    fn pop(&mut self, stack_num: i32) -> i32 {
        let stack_num = stack_num as usize;
        let start_idx = stack_num * self.stack_size;
        let stack_idx = self.stack_idxs[stack_num];
        if stack_idx == start_idx {
            return -1;
        }
        let result = self.stack[stack_idx - 1];
        self.stack_idxs[stack_num] -= 1;
        result
    }

    fn peek(&self, stack_num: i32) -> i32 {
        let stack_num = stack_num as usize;
        let start_idx = stack_num * self.stack_size;
        let stack_idx = self.stack_idxs[stack_num];
        if stack_idx == start_idx {
            return -1;
        }
        self.stack[stack_idx - 1]
    }

    fn is_empty(&self, stack_num: i32) -> bool {
        let stack_num = stack_num as usize;
        let start_idx = stack_num * self.stack_size;
        let stack_idx = self.stack_idxs[stack_num];
        stack_idx == start_idx
    }
}
