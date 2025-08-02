pub fn stars(n: u32) -> String {
    let mut res = "".to_string();
    for _i in 0..(2_i32).pow(n) {
        res += "*";
    }
    res
}
