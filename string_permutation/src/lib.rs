pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut sorted = s1.chars().collect::<Vec<char>>();
    let mut sorted2 = s2.chars().collect::<Vec<char>>();
    sorted.sort();
    sorted2.sort();
    sorted == sorted2
}
