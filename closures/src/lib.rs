pub fn first_fifty_even_square() -> Vec<i32> {
    (1..=100)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect()
}
