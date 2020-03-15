impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut minprice = std::i32::MAX;
    let mut maxprofit = 0;
    for &price in &prices {
      if price < minprice {
        minprice = price;
      } else if price - minprice > maxprofit {
        maxprofit = price - minprice;
      }
    }
    maxprofit
  }
}

struct Solution;

fn main() {
  println!("Hello, world!");
  let prices = vec![7, 1, 5, 3, 6, 4];
  let result = Solution::max_profit(prices);
  println!("result = {}", result);
}
