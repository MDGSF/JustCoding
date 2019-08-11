/*
 * 面试题 3： 二维数组中的查找
 * 题目：在一个二维数组中，每一行都按照从左到右递增的顺序排序，
 * 每一列都按照从上到下递增的顺序排序。请完成一个函数，输入这样的一个
 * 二维数组和一个整数，判断数组中是否含有该整数。
 *
 * 例子：
 * 1   2   8   9
 * 2   4   9   12
 * 4   7   10  13
 * 6   8   11  15
 *
 * 上面的二维数组就是每行、每列都递增排序。如果在这个数组中
 * 查找数字 7，则返回 true。如果查找数字 5，由于数组不含有该数字，
 * 则返回 false。
 *
 * 分析：
 * 我们现在要在上面的那个二维数组中查找数字 7。
 *
 * 我们先看右上角的数字 9，因为 9 > 7，所以 9 所在的这一列都不可能，
 * 那我们就删除掉这列，就只剩下 4 行 3 列了。
 *
 * 1   2   8   
 * 2   4   9   
 * 4   7   10  
 * 6   8   11  
 *
 * 然后我们继续看右上角的数字 8，因为 8 > 7，所以 8 所在的这一列都不可能，
 * 我们把这一列也删掉，于是就只剩下 4 行 2 列了。
 *
 * 1   2     
 * 2   4     
 * 4   7     
 * 6   8     
 *
 * 继续看右上角的数字 2，因为 2 < 7，所以 2 又是当前这一行最大的数字，
 * 所以当前行也不可能存在 7，那我们把 2 所在的这一行删掉，于是就只
 * 剩下 3 行 2 列了。
 *
 * 2   4     
 * 4   7     
 * 6   8     
 *
 * 继续看右上角的数字 4，因为 4 < 7，所以 4 又是当前这一行最大的数字，
 * 所以当前行也不可能存在 7，那我们把 4 所在的这一行删掉，于是就只
 * 剩下 2 行 2 列了。
 *
 * 4   7     
 * 6   8     
 *
 * 继续看右上角的数字 7，终于找到我们要的数字了，结束。
 *
 * 总结：
 * 我们先选取数组右上角的数字。
 * 1. 如果该数字等于要查找的数字，查找过程结束。
 * 2. 如果该数字大于要查找的数字，删除该数字所在的列。
 * 3. 如果该数字小于要查找的数字，删除该数字所在的行。
 *
 * 也就是说如果要查找的数字不在数组的右上角，则每次都在
 * 数组的查找范围中删除一行或者一列，这样每一步都可以缩小
 * 查找的范围，直到找到要查找的数字，或者查找范围为空。
 *
 */

fn find(matrix: Vec<Vec<i32>>, number: i32) -> bool {
    let rows = matrix.len() as i32;
    if rows == 0 {
        return false;
    }

    let columns = matrix[0].len() as i32;
    if columns == 0 {
        return false;
    }

    let mut row: i32 = 0;
    let mut column: i32 = columns - 1;

    while row < rows && column >= 0 {
        if matrix[row as usize][column as usize] == number {
            return true;
        } else if matrix[row as usize][column as usize] > number {
            column -= 1;
        } else {
            row += 1;
        }
    }

    false
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(matrix: Vec<Vec<i32>>, number: i32, expected: bool) {
        let result = find(matrix, number);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_1() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 8, 9],
            vec![2, 4, 9, 12],
            vec![4, 7, 10, 13],
            vec![6, 8, 11, 15],
        ];
        let number = 7;
        let expected = true;
        test(matrix, number, expected);
    }

    #[test]
    fn test_2() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 8, 9],
            vec![2, 4, 9, 12],
            vec![4, 7, 10, 13],
            vec![6, 8, 11, 15],
        ];
        let number = 5;
        let expected = false;
        test(matrix, number, expected);
    }

    #[test]
    fn test_3() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 8, 9],
            vec![2, 4, 9, 12],
            vec![4, 7, 10, 13],
            vec![6, 8, 11, 15],
        ];
        let number = 1;
        let expected = true;
        test(matrix, number, expected);
    }

    #[test]
    fn test_4() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 8, 9],
            vec![2, 4, 9, 12],
            vec![4, 7, 10, 13],
            vec![6, 8, 11, 15],
        ];
        let number = 15;
        let expected = true;
        test(matrix, number, expected);
    }

    #[test]
    fn test_5() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 8, 9],
            vec![2, 4, 9, 12],
            vec![4, 7, 10, 13],
            vec![6, 8, 11, 15],
        ];
        let number = 0;
        let expected = false;
        test(matrix, number, expected);
    }

    #[test]
    fn test_6() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 8, 9],
            vec![2, 4, 9, 12],
            vec![4, 7, 10, 13],
            vec![6, 8, 11, 15],
        ];
        let number = 16;
        let expected = false;
        test(matrix, number, expected);
    }

    #[test]
    fn test_7() {
        let matrix: Vec<Vec<i32>> = Vec::new();
        let number = 16;
        let expected = false;
        test(matrix, number, expected);
    }
}
