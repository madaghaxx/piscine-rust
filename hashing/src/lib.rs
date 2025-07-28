pub fn mean(list: &[i32]) -> f64 {
    let mut res: f64 = 0.0;
    for num in list {
        res += *num as f64;
    }
    res / (list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut lst: Vec<i32> = list.to_vec();

    lst.sort();
    if lst.len() % 2 == 0 {
        return (lst[lst.len() / 2 - 1] + lst[lst.len() / 2]) / 2;
    }
    lst[lst.len() / 2]
}
use std::collections::HashMap;
pub fn mode(list: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    for &num in list {
        *counts.entry(num).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .unwrap_or(0)
}
