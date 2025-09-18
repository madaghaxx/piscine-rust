pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut res: Vec<Box<u32>> = Vec::new();
    let ss = s.split_whitespace();
    for n in ss {
        if n.ends_with('k') {
            let trimmed = n.trim_end_matches('k');
            res.push(Box::new((trimmed.parse::<f32>().unwrap() * 1000.0)as u32));
        } else {
            res.push(Box::new((n.parse::<f32>().unwrap())as u32));
        }
    }
    res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut res : Vec<u32>=vec![];
    for i in a{
        res.push(*i);
    }
    res
}
