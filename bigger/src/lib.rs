use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut res = 0;
    for (s, i) in h {
        if i > res {
            res = i;
        }
    }
    res
}
