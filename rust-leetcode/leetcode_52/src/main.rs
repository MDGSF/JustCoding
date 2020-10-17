impl Solution {
  pub fn total_n_queens(n: i32) -> i32 {
    let mut count = 0;
    Solution::dfs(0, 0, 0, 0, n, &mut count);
    count
  }

  fn dfs(cols: i32, pie: i32, na: i32, row: i32, n: i32, mut count: &mut i32) {
    if row >= n {
      *count += 1;
      return;
    }

    // 得到当前的所有空位
    let mut bits = (!(cols | pie | na)) & ((1 << n) - 1);
    while bits != 0 {
      // 取到最低位的 1
      let p = bits & -bits;

      // 在 p 位置放入皇后
      bits = bits & (bits - 1);

      Solution::dfs(
        cols | p,
        (pie | p) << 1,
        (na | p) >> 1,
        row + 1,
        n,
        &mut count,
      );
    }
  }
}

struct Solution;

fn main() {
  let result = Solution::total_n_queens(4);
  println!("result = {}", result);
}
