/*
 * 面试题 4： 替换空格
 * 题目：请实现一个函数，把字符串中的每个空格替换成 "%20"。
 * 例如输入 "We are happy"，则输出 "We%20are%20happy"。
 *
 * 分析：
 * 我们可以先遍历一遍字符串，计算出空格的数量。
 * 因为每个空格要被替换为 "%20"，也就是需要 3 倍的空间。
 * 所以 
 *   新的字符串长度 = 旧的字符串长度 + 空格数量 * 2
 *
 * 然后从后往前复制。碰到非空格字符就直接复制，碰到空格就
 * 把 "%20" 这 3 个字符复制到新的字符串中。
 *
 * 例子：
 * input:  "hello world"
 * output: "hello%20world"
 *
 * 1. 计算出空格数量 = 1
 * 2. 新的字符串长度 = 旧的字符串长度 + 空格数量 * 2
 *                   = 11 + 1 * 2
 *                   = 13
 * 3. 从后往前复制。
 *  旧：            ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
 *  新：  ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_']
 *  下标: [ 0 ,  1 ,  2 ,  3 ,  4 ,  5 ,  6 ,  7 ,  8 ,  9 ,  10,  11,  12]
 *
 *  3.1 先复制最后一个字母 'd'，因为不是空格，就直接复制，得到如下：
 *  旧：            ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
 *  新：  ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'd']
 *  下标: [ 0 ,  1 ,  2 ,  3 ,  4 ,  5 ,  6 ,  7 ,  8 ,  9 ,  10,  11,  12]
 *
 *  3.2 然后一直往前
 *  旧：            ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
 *  新：  ['_', '_', '_', '_', '_', '_', '_', '_', 'w', 'o', 'r', 'l', 'd']
 *  下标: [ 0 ,  1 ,  2 ,  3 ,  4 ,  5 ,  6 ,  7 ,  8 ,  9 ,  10,  11,  12]
 *
 *  3.3 终于碰到空格了，用 "%20" 这 3 个字母替换
 *  旧：            ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
 *  新：  ['_', '_', '_', '_', '_', '%', '2', '0', 'w', 'o', 'r', 'l', 'd']
 *  下标: [ 0 ,  1 ,  2 ,  3 ,  4 ,  5 ,  6 ,  7 ,  8 ,  9 ,  10,  11,  12]
 *
 *  3.4 然后继续替换字母 'o'
 *  旧：            ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
 *  新：  ['_', '_', '_', '_', 'o', '%', '2', '0', 'w', 'o', 'r', 'l', 'd']
 *  下标: [ 0 ,  1 ,  2 ,  3 ,  4 ,  5 ,  6 ,  7 ,  8 ,  9 ,  10,  11,  12]
 *
 */
fn replace_blank(arr: &[u8]) -> Option<Vec<u8>> {
    if arr.len() == 0 {
        return None;
    }

    let mut number_of_blank = 0;
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == b' ' {
            number_of_blank += 1;
        }
        i += 1;
    }

    let new_length = arr.len() + number_of_blank * 2;
    let mut result: Vec<u8> = vec![0; new_length];

    let mut index_of_origin = (arr.len() - 1) as i32;
    let mut index_of_new = (new_length - 1) as i32;

    while index_of_origin >= 0 {
        if arr[index_of_origin as usize] == b' ' {
            result[index_of_new as usize] = b'0';
            result[(index_of_new - 1) as usize] = b'2';
            result[(index_of_new - 2) as usize] = b'%';
            index_of_new -= 3;
        } else {
            result[index_of_new as usize] = arr[index_of_origin as usize];
            index_of_new -= 1;
        }
        index_of_origin -= 1;
    }

    Some(result)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let arr = vec![b'a', b'b'];
        let result = replace_blank(&arr[..]); 
        assert_eq!(result, Some(arr));
    }

    #[test]
    fn test2() {
        let s = String::from("hello world");
        let expected = String::from("hello%20world");
        let result = replace_blank(s.as_bytes()); 
        if let Some(val) = result {
            assert_eq!(val, expected.as_bytes());
        };
    }

    #[test]
    fn test3() {
        let s = String::from(" helloworld");
        let expected = String::from("%20helloworld");
        let result = replace_blank(s.as_bytes()); 
        if let Some(val) = result {
            assert_eq!(val, expected.as_bytes());
        };
    }

    #[test]
    fn test4() {
        let s = String::from("hello  world");
        let expected = String::from("hello%20%20world");
        let result = replace_blank(s.as_bytes()); 
        if let Some(val) = result {
            assert_eq!(val, expected.as_bytes());
        };
    }

    #[test]
    fn test5() {
        let s = String::from("");
        let result = replace_blank(s.as_bytes()); 
        assert_eq!(result, None);
    }

    #[test]
    fn test6() {
        let s = String::from(" ");
        let expected = String::from("%20");
        let result = replace_blank(s.as_bytes()); 
        if let Some(val) = result {
            assert_eq!(val, expected.as_bytes());
        };
    }

    #[test]
    fn test7() {
        let s = String::from("helloworld");
        let expected = String::from("helloworld");
        let result = replace_blank(s.as_bytes()); 
        if let Some(val) = result {
            assert_eq!(val, expected.as_bytes());
        };
    }

    #[test]
    fn test8() {
        let s = String::from("   ");
        let expected = String::from("%20%20%20");
        let result = replace_blank(s.as_bytes()); 
        if let Some(val) = result {
            assert_eq!(val, expected.as_bytes());
        };
    }
}
