use std::collections::HashMap;

impl Solution {
  pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
      return false;
    }

    let s = s.into_bytes();
    let t = t.into_bytes();
    let mut ms = HashMap::new();
    let mut mt = HashMap::new();
    let mut cs = 0;
    let mut ct = 0;
    for i in 0..s.len() {
      if !ms.contains_key(&s[i]) {
        cs += 1;
        ms.insert(s[i], cs);
      }
      if !mt.contains_key(&t[i]) {
        ct += 1;
        mt.insert(t[i], ct);
      }
      if ms.get(&s[i]) != mt.get(&t[i]) {
        return false;
      }
    }

    true
  }
}

pub struct Solution;
