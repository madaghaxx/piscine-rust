pub fn factorial(num: u64) -> u64 {
    let mut  res: u64 = 1;
    for i in 2..=num {
        res *= i;
    }
    res
}
