use std::collections::HashSet;

impl Solution {
    pub fn adventure_camp(expeditions: Vec<String>) -> i32 {
        let expeditions_sets = expeditions
            .iter()
            .map(|expedition| {
                expedition
                    .split("->")
                    .filter(|camp| !camp.is_empty())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<HashSet<_>>>();

        println!("{:?}", expeditions_sets);

        let mut all_sets = expeditions_sets[0].iter().cloned().collect::<HashSet<_>>();
        let n = expeditions.len();
        let mut max_count = 0;
        let mut max_idx = -1;
        for i in 1..n {
            let count = expeditions_sets[i].difference(&all_sets).count();
            println!("i:{}, count:{}, max_count:{}", i, count, max_count);
            if count > max_count {
                max_count = count;
                max_idx = i as i32;
            }
            all_sets = all_sets.union(&expeditions_sets[i]).cloned().collect();
        }
        max_idx
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let expeditions = vec![
            "leet->code".to_string(),
            "leet->code->Campsite->Leet".to_string(),
            "leet->code->leet->courier".to_string(),
        ];
        assert_eq!(Solution::adventure_camp(expeditions), 1);
    }

    #[test]
    fn test02() {
        let expeditions = vec!["Alice->Dex".to_string(), "".to_string(), "Dex".to_string()];
        assert_eq!(Solution::adventure_camp(expeditions), -1);
    }

    #[test]
    fn test03() {
        let expeditions = vec![
            "".to_string(),
            "Gryffindor->Slytherin->Gryffindor".to_string(),
            "Hogwarts->Hufflepuff->Ravenclaw".to_string(),
        ];
        assert_eq!(Solution::adventure_camp(expeditions), 2);
    }

    #[test]
    fn test04() {
        let expeditions = vec![
            "xO->xO->xO".to_string(),
            "xO->BKbWDH".to_string(),
            "xO->BKbWDH".to_string(),
            "BKbWDH->mWXW".to_string(),
            "LSAYWW->LSAYWW".to_string(),
            "oAibBvPdJ".to_string(),
            "LSAYWW->u".to_string(),
            "LSAYWW->LSAYWW".to_string(),
        ];
        assert_eq!(Solution::adventure_camp(expeditions), 1);
    }
}
