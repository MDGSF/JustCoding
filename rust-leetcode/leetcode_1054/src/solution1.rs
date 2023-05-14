impl Solution {
    pub fn rearrange_barcodes(mut barcodes: Vec<i32>) -> Vec<i32> {
        barcodes.sort();
        let mut prev: usize = 0;
        let mut back: usize = (barcodes.len() + 1) / 2;
        let mut res: Vec<i32> = barcodes
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                if i & 1 == 0 {
                    prev += 1;
                    barcodes[prev - 1]
                } else {
                    back += 1;
                    barcodes[back - 1]
                }
            })
            .collect();
        let n = res.len();
        for i in 0..(n - 1) {
            if res[i] != res[i + 1] {
                continue;
            }
            res.remove(i);
            res.insert(0, res[i]);
        }
        res
    }
}

pub struct Solution;
