use std::collections::HashMap;
use std::collections::BinaryHeap;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut result = vec![];

        for &code in barcodes.iter() {
            *counter.entry(code).or_default() += 1;
        }
        for (&k, &v) in counter.iter() {
            heap.push((v, k));
        }

        while let Some(mut max0) = heap.pop() {
            result.push(max0.1);
            max0.0 -= 1;

            if let Some(mut max1) = heap.pop() {
                result.push(max1.1);
                max1.0 -= 1;
                if max1.0 > 0 {
                    heap.push(max1);
                }
            }

            if max0.0 > 0 {
                heap.push(max0);
            }
        }

        result
    }
}

pub struct Solution;