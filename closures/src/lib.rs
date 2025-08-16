pub fn first_fifty_even_square() -> Vec<i32> {
    (2..)
        .filter(|e| e % 2 == 0)
        .map(|e: i32| e.pow(2))
        .take(50)
        .collect()
}
