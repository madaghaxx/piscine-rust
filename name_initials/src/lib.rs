pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for name in &names {
        let arr: Vec<&str> = name.split_whitespace().collect();
        if arr.len() >= 2 {
            res.push(
                format!(
                    "{}. {}.",
                    arr[0].chars().next().unwrap_or_default(),
                    arr[1].chars().next().unwrap_or_default()
                )
            );
        }
    }
    res
}
