// pub fn is_empty(v: &str) -> bool {
//     if v.len() == 0 { true } else { false }
// }

// pub fn is_ascii(v: &str) -> bool {
//     str_len(v) == v.len()
// }

// pub fn contains(v: &str, pat: &str) -> bool {
//     v.contains(pat)
// }

// pub fn split_at(v: &str, index: usize) -> (&str, &str) {
//     // v.split_at(index)
//     (&v[0..index], &v[index..v.len() - 1])
// }

// pub fn find(v: &str, pat: char) -> usize {
//     let mut res = 0;
//     for i in 0..v.len() {
//         if v[i] == pat {
//             return i;
//         }
//     }
//     0
// }

// pub fn str_len(s: &str) -> usize {
//     let arr = s.clone().chars().collect::<Vec<char>>();
//     arr.len()
// }

pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("kys")
}
