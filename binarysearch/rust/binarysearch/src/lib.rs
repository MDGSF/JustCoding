pub fn binarysearch_first(a: &[i32], v: i32) -> i32 {
  if a.len() == 0 {
    return -1;
  }
  let mut low = 0;
  let mut high = a.len() - 1;
  while low <= high {
    let mid = low + (high - low) / 2;
    if a[mid] > v {
      high = mid - 1;
    } else if a[mid] > v {
      low = mid + 1;
    } else {
      if mid == 0 || a[mid - 1] != v {
        return mid as i32;
      } else {
        high = mid - 1;
      }
    }
  }
  return -1;
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
