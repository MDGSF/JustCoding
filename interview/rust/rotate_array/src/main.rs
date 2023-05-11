fn main() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let k = 3;
    rotate(&mut arr, k);
    println!("{:?}", arr);
}

fn rotate(arr: &mut [i32], k: usize) {
    let n = arr.len();
    let k = k % n;
    arr[0..k].reverse();
    arr[k..n].reverse();
    arr[0..n].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let k = 3;
        rotate(&mut arr, k);
        assert_eq!(arr, vec![4, 5, 6, 7, 8, 9, 1, 2, 3]);
    }
}
