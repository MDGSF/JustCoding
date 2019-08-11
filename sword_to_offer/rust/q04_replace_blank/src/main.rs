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
