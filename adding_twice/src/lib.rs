pub fn add_curry(n: i32) -> impl Fn(i32) -> i32 {
    move |i| i + n
}

pub fn twice(f: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f(f(x))
}
