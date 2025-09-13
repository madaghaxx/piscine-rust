pub fn num_to_ordinal(x: u32) -> String {
    let mut res = x.to_string();
    match x {
        11 | 12 | 13 => {
            res += "th";
        }
        _ =>
            match x % 10 {
                1 => {
                    res += "st";
                }
                2 => {
                    res += "nd";
                }
                3 => {
                    res += "rd";
                }
                _ => {
                    res += "th";
                }
            }
    }

    res
}
