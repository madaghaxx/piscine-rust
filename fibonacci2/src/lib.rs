pub fn fibonacci(n: u32) -> u32 {
    if n == 0 { 0 } else { fibonacci(n - 1) + fibonacci(n + 1) }
}
