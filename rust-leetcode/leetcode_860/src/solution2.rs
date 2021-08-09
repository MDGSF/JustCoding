impl Solution {
  pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut five = 0;
    let mut ten = 0;
    for &bill in bills.iter() {
      match (bill, five > 0, ten > 0, five >= 3) {
        (5, _, _, _) => five += 1,
        (10, true, _, _) => {
          five -= 1;
          ten += 1;
        }
        (20, true, true, _) => {
          ten -= 1;
          five -= 1;
        }
        (20, _, _, true) => five -= 3,
        _ => return false,
      }
    }
    true
  }
}

pub struct Solution;
