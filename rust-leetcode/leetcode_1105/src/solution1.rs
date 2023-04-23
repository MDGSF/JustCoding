/*
books[i][0]: 第 i 本书的厚度
books[i][1]: 第 i 本书的高度
dp[i]: 表示遍历到第 i 个时，最小高度。
dp[i] = min(dp[j] + max(books[k])), 0 <= j <= k < i <= n, sum(books[k]) < shelf_width
*/
impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![1000000; n + 1];
        dp[0] = 0;
        for i in 0..n {
            let mut max_height = 0;
            let mut cur_width = 0;
            for j in (0..=i).rev() {
                cur_width += books[j][0];
                if cur_width > shelf_width {
                    break;
                }
                max_height = max_height.max(books[j][1]);
                dp[i + 1] = dp[i + 1].min(dp[j] + max_height);
            }
        }
        dp[n]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let books = [[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]];
        let books = books
            .into_iter()
            .map(|book| vec![book[0], book[1]])
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_height_shelves(books, 4), 6);
    }

    #[test]
    fn test02() {
        let books = [[1, 3], [2, 4], [3, 2]];
        let books = books
            .into_iter()
            .map(|book| vec![book[0], book[1]])
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_height_shelves(books, 6), 4);
    }
}
