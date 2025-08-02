pub fn number_logic(num: u32) -> bool {
    let s = num.to_string();
    let pow = s.len() as u32;
    let mut res = 0u32;
    for i in 0..s.len() {
        let n = s.chars().nth(i).unwrap().to_digit(10).unwrap();
        res += n.pow(pow);
    }
    res == num
}
