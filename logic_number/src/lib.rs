pub fn number_logic(num: u32) -> bool {
    let s = num.to_string();
    let n = s.len() as u32;
    s.chars()
        .map(|c| c.to_digit(10).unwrap().pow(n))
        .sum::<u32>() == num
}
