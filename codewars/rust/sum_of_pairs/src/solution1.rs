use std::collections::HashSet;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
  let mut sets = HashSet::new();
  for &i in ints {
    let peer = s - i;
    if sets.contains(&peer) {
      return Some((peer, i));
    } else {
      sets.insert(i);
    }
  }
  None
}

#[test]
fn returns_expected() {
  let l1 = [1, 4, 8, 7, 3, 15];
  let l2 = [1, -2, 3, 0, -6, 1];
  let l3 = [20, -13, 40];
  let l4 = [1, 2, 3, 4, 1, 0];
  let l5 = [10, 5, 2, 3, 7, 5];
  let l6 = [4, -2, 3, 3, 4];
  let l7 = [0, 2, 0];
  let l8 = [5, 9, 13, -3];
  assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
  assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
  assert_eq!(sum_pairs(&l3, -7), None);
  assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
  assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
  assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
  assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
  assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}
