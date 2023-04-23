use adventure_camp::solution1::Solution;

fn main() {
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

    let result = Solution::adventure_camp(expeditions);
    println!("result: {}", result);
}
