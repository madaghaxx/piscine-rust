pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let arr = s.split(" ");
    let mut res: Vec<Box<u32>> = vec![];
    for i in arr {
        if i.ends_with('k') {
            let mut temp = i.to_string();
            temp.pop();
            if let Ok(n) = temp.parse::<f32>() {
                let num = (n * 1000.0) as u32;
                res.push(Box::new(num));
            }
        } else {
            if let Ok(n) = i.parse::<u32>() {
                res.push(Box::new(n));
            }
        }
    }
    res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut res :Vec<u32>= vec![];
    for b in a {
        res.push(*b);
    }
    res
}
