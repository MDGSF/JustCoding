use std::collections::HashMap;

impl Solution {
  pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    if s.len() == 0 || words.len() == 0 {
      return Vec::new();
    }
    let one_word_len = words[0].len();
    let sub_str_len = one_word_len * words.len();
    let s_len = s.len();
    if s_len < sub_str_len {
      return Vec::new();
    }
    let mut words_counter: HashMap<&str, i32> = HashMap::new();
    for word in words.iter() {
      *words_counter.entry(word).or_insert(0) += 1;
    }
    let mut result = Vec::new();
    for i in 0..=(s_len - sub_str_len) {
      let temp_str = &s[i..i + sub_str_len];
      let mut temp_words = Vec::new();
      for j in (0..sub_str_len).step_by(one_word_len) {
        temp_words.push(&temp_str[j..j + one_word_len]);
      }

      let mut temp_words_counter: HashMap<&str, i32> = HashMap::new();
      for word in temp_words.iter() {
        *temp_words_counter.entry(word).or_insert(0) += 1;
      }
      if words_counter == temp_words_counter {
        result.push(i as i32);
      }
    }
    result
  }
}

struct Solution;

fn main() {
  //let s = "barfoothefoobarman".to_string();
  //let words = vec!["foo".to_string(), "bar".to_string()];
  let s = "a".to_string();
  let words = vec!["a".to_string(), "a".to_string()];
  let result = Solution::find_substring(s, words);
  println!("result = {:?}", result);
}
