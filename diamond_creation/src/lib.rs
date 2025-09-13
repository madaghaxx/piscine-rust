fn make_line(i: usize, n: usize) -> String {
    let width = 2 * n + 1;
    let ch = (b'A' + i as u8) as char;
    let lead = " ".repeat(n - i);
    let mut s = String::with_capacity(width);
    s.push_str(&lead);
    if i == 0 {
        s.push(ch);
    } else {
        s.push(ch);
        s.push_str(&" ".repeat(2 * i - 1));
        s.push(ch);
    }
    s.push_str(&lead);
    s
}
pub fn get_diamond(c: char) -> Vec<String> {
    let c = c.to_ascii_uppercase();
    if !(('A'..='Z').contains(&c)) {
        return Vec::new();
    }
    let n = (c as u8 - b'A') as usize;
    let mut rows: Vec<String> = (0..=n).map(|i| make_line(i, n)).collect();
    rows.extend((0..n).rev().map(|i| make_line(i, n)));
    rows
}
